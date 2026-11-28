# 0005: Image format: EROFS/composefs vs plain overlay for v1

- Status: Accepted (signed off by the project owner, 2026-11-28)
- Date: 2026-11-28

## Context

See README §4.2. M2 must pull OCI images and give each container a root filesystem with a private writable layer. Many clusters and pods on one CI runner share the same images, so the memory and disk win comes from sharing layers and page cache across them.

Forces:
- Overlayfs is in every kernel we target, and identical lower directories already share page cache: two containers on the same lower layer read the same inodes.
- EROFS/composefs adds **file-level** dedup across different layers and images (identical files in unrelated images share one backing file), plus fs-verity. It needs the `erofs` kernel module, overlayfs data-only lower layers (Linux 6.5+), and building EROFS metadata images (a Rust writer or `mkfs.erofs`).
- Whether `erofs` loads on GitHub's hosted runner kernel (7.0 azure) has not been checked yet. The orb kernel doesn't list it, but orbs can't mount anything anyway (ADR 0004).
- The image store is shared across runs on a host, so its on-disk layout is a compatibility surface once people cache it in CI.

## Options

1. **Plain overlay over extracted layers.** A content-addressed store of layer tarballs and their extracted directories, keyed by diffID. Each container's rootfs is an overlay of its layers' directories plus a per-container upper and work dir. Simple, works everywhere, shares page cache per layer.
2. **EROFS/composefs from the start.** Files stored once by content hash; each image gets an EROFS metadata image that points at them, mounted as the overlay's lower layer. Best sharing, more kernel and tooling requirements, and more code before the first pod runs.
3. **One flattened directory per image.** Fewer overlay layers, but duplicates shared base layers across images, losing most of the sharing.

## Decision

**Option 1 for v1, with EROFS/composefs (option 2) as a later optimisation behind the same interface.**

- **Store layout** under a host-wide directory (default `/var/lib/kubemu/images`, configurable): `blobs/sha256/<digest>` for compressed blobs, `layers/<diffid>/` for extracted trees, and image manifests and configs indexed by reference and digest. Writes go to a temp name and are renamed into place, so concurrent kubemu processes and crashes don't corrupt it.
- **Extraction** handles whiteouts and opaque directories, converting them to overlayfs's form (character devices 0:0, `trusted.overlay.opaque`), and preserves ownership and modes (we are root, ADR 0004).
- **Rootfs:** overlay with lower layers in image order, upper and work dirs per container under the cluster's state directory, removed with the pod.
- **Pulling:** a Rust registry client (anonymous and basic or bearer token auth, multi-arch index resolution for the host arch). Lazy pulling stays a research track.
- **Manifest and reference logic** (resolution, digests, which layers a pod needs) is core code; downloads, extraction and mounts are shell.
- **The `ImageStore` interface** hides the layout: callers ask for \"a rootfs for this image at this path\". EROFS can replace the lower-layer half later without touching the executor.
- **Before EROFS work starts**, check `erofs` and data-only lower layers on the CI runners and record the result in a follow-up ADR.

## Consequences

- The first real pod needs only overlayfs, tar and gzip/zstd decoding, so M2 reaches running workloads sooner.
- Sharing is per layer, not per file. Images built on the same base share it; unrelated images with identical files don't. The budget in §4.4 (ten clusters with the reference app on 4 vCPU / 8 GB) should still hold, since those clusters run the same images; benchmarks will tell.
- Deep images can hit overlay's lower-layer limit (around 500 with short paths); kubemu reports it as an image error. Rare for real images.
- A garbage collector for unused layers is needed eventually; for M2, the store only grows and `kubemu` gets a prune command later.
- Switching to EROFS later changes the on-disk layout. The store gets a version file from day one so a later kubemu can migrate or rebuild it.
