
2.0

KubernetesunversionedBçÖ
Ó
/api/‰·
coreget available API versions*getCoreAPIVersions2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJl
Q
200J
H
OKB
@
>#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions

401

UnauthorizedRhttps¯
/api/v1/ÎË
core_v1get available resources*getCoreV1APIResources2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJp
U
200N
L
OKF
D
B#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList

401

UnauthorizedRhttpsÓ
/api/v1/configmaps◊
core_v1'list or watch objects of kind ConfigMap*#listCoreV1ConfigMapForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*J\
A
200:
8
OK2
0
.#/definitions/io.k8s.api.core.v1.ConfigMapList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÌ
/api/v1/endpoints◊
core_v1'list or watch objects of kind Endpoints*#listCoreV1EndpointsForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*J\
A
200:
8
OK2
0
.#/definitions/io.k8s.api.core.v1.EndpointsList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK⁄
/api/v1/events«‡
core_v1#list or watch objects of kind Event*listCoreV1EventForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*JX
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.EventList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK∑
/api/v1/namespaces†¢
core_v1'list or watch objects of kind Namespace*listCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJ\
A
200:
8
OK2
0
.#/definitions/io.k8s.api.core.v1.NamespaceList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
"÷
core_v1create a Namespace*createCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.NamespaceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J·
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Namespace
C
202<
:
Accepted.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
J 
#/parameters/pretty-tJGM1-ng≠!
)/api/v1/namespaces/{namespace}/configmapsˇ ¨
core_v1'list or watch objects of kind ConfigMap*listCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJ\
A
200:
8
OK2
0
.#/definitions/io.k8s.api.core.v1.ConfigMapList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
"‡
core_v1create a ConfigMap*createCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.ConfigMapBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J·
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap
C
202<
:
Accepted.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
*£

core_v1delete collection of ConfigMap*)deleteCoreV1CollectionNamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngã*
0/api/v1/namespaces/{namespace}/configmaps/{name}÷)Û
core_v1read the specified ConfigMap*readCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JX
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
®
core_v1replace the specified ConfigMap* replaceCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.ConfigMapBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
*Ê
core_v1delete a ConfigMap*deleteCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
B 
core_v1(partially update the specified ConfigMap*patchCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
J6
42"0pathname of the ConfigMap"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ng™!
(/api/v1/namespaces/{namespace}/endpoints˝ ¨
core_v1'list or watch objects of kind Endpoints*listCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJ\
A
200:
8
OK2
0
.#/definitions/io.k8s.api.core.v1.EndpointsList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
"ﬁ
core_v1create Endpoints*createCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.EndpointsBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J·
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Endpoints
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Endpoints
C
202<
:
Accepted.
,
*#/definitions/io.k8s.api.core.v1.Endpoints

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
*£

core_v1delete collection of Endpoints*)deleteCoreV1CollectionNamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngà*
//api/v1/namespaces/{namespace}/endpoints/{name}‘)Û
core_v1read the specified Endpoints*readCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JX
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Endpoints

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
®
core_v1replace the specified Endpoints* replaceCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.EndpointsBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Endpoints
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Endpoints

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
*‰
core_v1delete Endpoints*deleteCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
B 
core_v1(partially update the specified Endpoints*patchCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Endpoints
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Endpoints

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
J6
42"0pathname of the Endpoints"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngÚ 
%/api/v1/namespaces/{namespace}/events» ú
core_v1#list or watch objects of kind Event*listCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJX
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.EventList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
"≈
core_v1create an Event*createCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B<
:
8bodybody *(
&#/definitions/io.k8s.api.core.v1.EventBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J’
9
2002
0
OK*
(
&#/definitions/io.k8s.api.core.v1.Event
>
2017
5
Created*
(
&#/definitions/io.k8s.api.core.v1.Event
?
2028
6
Accepted*
(
&#/definitions/io.k8s.api.core.v1.Event

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
*ó

core_v1delete collection of Event*%deleteCoreV1CollectionNamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngº)
,/api/v1/namespaces/{namespace}/events/{name}ã)„
core_v1read the specified Event*readCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JT
9
2002
0
OK*
(
&#/definitions/io.k8s.api.core.v1.Event

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
ê
core_v1replace the specified Event*replaceCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B<
:
8bodybody *(
&#/definitions/io.k8s.api.core.v1.EventBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jî
9
2002
0
OK*
(
&#/definitions/io.k8s.api.core.v1.Event
>
2017
5
Created*
(
&#/definitions/io.k8s.api.core.v1.Event

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
*€
core_v1delete an Event*deleteCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
B∂
core_v1$partially update the specified Event*patchCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJî
9
2002
0
OK*
(
&#/definitions/io.k8s.api.core.v1.Event
>
2017
5
Created*
(
&#/definitions/io.k8s.api.core.v1.Event

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
J2
0.",pathname of the Event"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ng” 
#/api/v1/namespaces/{namespace}/pods´ î
core_v1!list or watch objects of kind Pod*listCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJV
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.PodList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
"∂
core_v1create a Pod*createCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B:
8
6bodybody *&
$#/definitions/io.k8s.api.core.v1.PodBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jœ
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod
<
2015
3
Created(
&
$#/definitions/io.k8s.api.core.v1.Pod
=
2026
4
Accepted(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
*ë

core_v1delete collection of Pod*#deleteCoreV1CollectionNamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngÈ(
*/api/v1/namespaces/{namespace}/pods/{name}∫(€
core_v1read the specified Pod*readCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JR
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
Ñ
core_v1replace the specified Pod*replaceCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B:
8
6bodybody *&
$#/definitions/io.k8s.api.core.v1.PodBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jê
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod
<
2015
3
Created(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
*™
core_v1delete a Pod*deleteCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJë
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod
=
2026
4
Accepted(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
B¨
core_v1"partially update the specified Pod*patchCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJê
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod
<
2015
3
Created(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J0
.,"*pathname of the Pod"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngì
2/api/v1/namespaces/{namespace}/pods/{name}/binding‹"¥
core_v1create binding of a Pod* createCoreV1NamespacedPodBinding2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B>
<
:bodybody **
(#/definitions/io.k8s.api.core.v1.BindingJ€
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Binding
@
2019
7
Created,
*
(#/definitions/io.k8s.api.core.v1.Binding
A
202:
8
Accepted,
*
(#/definitions/io.k8s.api.core.v1.Binding

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jI
x-kubernetes-group-version-kind&$group: ""
kind: Binding
version: v1
Jû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†J&$
"#/parameters/fieldManager-Qy4HdaTWJ‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J4
20".pathname of the Binding"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngÛ!
1/api/v1/namespaces/{namespace}/pods/{name}/statusΩ!Î
core_v1 read status of the specified Pod*readCoreV1NamespacedPodStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JR
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
î
core_v1#replace status of the specified Pod* replaceCoreV1NamespacedPodStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B:
8
6bodybody *&
$#/definitions/io.k8s.api.core.v1.PodBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jê
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod
<
2015
3
Created(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
Bº
core_v1,partially update status of the specified Pod*patchCoreV1NamespacedPodStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJê
7
2000
.
OK(
&
$#/definitions/io.k8s.api.core.v1.Pod
<
2015
3
Created(
&
$#/definitions/io.k8s.api.core.v1.Pod

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J0
.,"*pathname of the Pod"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngÄ!
&/api/v1/namespaces/{namespace}/secrets’ †
core_v1$list or watch objects of kind Secret*listCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJY
>
2007
5
OK/
-
+#/definitions/io.k8s.api.core.v1.SecretList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
"À
core_v1create a Secret*createCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B=
;
9bodybody *)
'#/definitions/io.k8s.api.core.v1.SecretBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jÿ
:
2003
1
OK+
)
'#/definitions/io.k8s.api.core.v1.Secret
?
2018
6
Created+
)
'#/definitions/io.k8s.api.core.v1.Secret
@
2029
7
Accepted+
)
'#/definitions/io.k8s.api.core.v1.Secret

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
*ö

core_v1delete collection of Secret*&deleteCoreV1CollectionNamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngœ)
-/api/v1/namespaces/{namespace}/secrets/{name}ù)Á
core_v1read the specified Secret*readCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JU
:
2003
1
OK+
)
'#/definitions/io.k8s.api.core.v1.Secret

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
ñ
core_v1replace the specified Secret*replaceCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B=
;
9bodybody *)
'#/definitions/io.k8s.api.core.v1.SecretBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jñ
:
2003
1
OK+
)
'#/definitions/io.k8s.api.core.v1.Secret
?
2018
6
Created+
)
'#/definitions/io.k8s.api.core.v1.Secret

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
*›
core_v1delete a Secret*deleteCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
Bª
core_v1%partially update the specified Secret*patchCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJñ
:
2003
1
OK+
)
'#/definitions/io.k8s.api.core.v1.Secret
?
2018
6
Created+
)
'#/definitions/io.k8s.api.core.v1.Secret

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
J3
1/"-pathname of the Secret"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ng¯!
./api/v1/namespaces/{namespace}/serviceaccounts≈!¿
core_v1,list or watch objects of kind ServiceAccount*"listCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJa
F
200?
=
OK7
5
3#/definitions/io.k8s.api.core.v1.ServiceAccountList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
"É
core_v1create a ServiceAccount*$createCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*BE
C
Abodybody *1
/#/definitions/io.k8s.api.core.v1.ServiceAccountBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J
B
200;
9
OK3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount
G
201@
>
Created3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount
H
202A
?
Accepted3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
*≤

core_v1#delete collection of ServiceAccount*.deleteCoreV1CollectionNamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ng€*
5/api/v1/namespaces/{namespace}/serviceaccounts/{name}°*á
core_v1!read the specified ServiceAccount*"readCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*J]
B
200;
9
OK3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
∆
core_v1$replace the specified ServiceAccount*%replaceCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*BE
C
Abodybody *1
/#/definitions/io.k8s.api.core.v1.ServiceAccountBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J¶
B
200;
9
OK3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount
G
201@
>
Created3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
*·
core_v1delete a ServiceAccount*$deleteCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJß
B
200;
9
OK3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount
H
202A
?
Accepted3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
B„
core_v1-partially update the specified ServiceAccount*#patchCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJ¶
B
200;
9
OK3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount
G
201@
>
Created3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
J;
97"5pathname of the ServiceAccount"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngè!
'/api/v1/namespaces/{namespace}/services„ §
core_v1%list or watch objects of kind Service*listCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJZ
?
2008
6
OK0
.
,#/definitions/io.k8s.api.core.v1.ServiceList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
"“
core_v1create a Service*createCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B>
<
:bodybody **
(#/definitions/io.k8s.api.core.v1.ServiceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J€
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service
@
2019
7
Created,
*
(#/definitions/io.k8s.api.core.v1.Service
A
202:
8
Accepted,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
*ù

core_v1delete collection of Service*'deleteCoreV1CollectionNamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ng¡)
./api/v1/namespaces/{namespace}/services/{name}é)Î
core_v1read the specified Service*readCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JV
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
ú
core_v1replace the specified Service*replaceCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B>
<
:bodybody **
(#/definitions/io.k8s.api.core.v1.ServiceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jò
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service
@
2019
7
Created,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
*æ
core_v1delete a Service*deleteCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJô
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service
A
202:
8
Accepted,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
B¿
core_v1&partially update the specified Service*patchCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJò
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service
@
2019
7
Created,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J4
20".pathname of the Service"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ng∑"
5/api/v1/namespaces/{namespace}/services/{name}/status˝!˚
core_v1$read status of the specified Service*!readCoreV1NamespacedServiceStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JV
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
¨
core_v1'replace status of the specified Service*$replaceCoreV1NamespacedServiceStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B>
<
:bodybody **
(#/definitions/io.k8s.api.core.v1.ServiceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jò
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service
@
2019
7
Created,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
B–
core_v10partially update status of the specified Service*"patchCoreV1NamespacedServiceStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJò
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.Service
@
2019
7
Created,
*
(#/definitions/io.k8s.api.core.v1.Service

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J4
20".pathname of the Service"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngß)
/api/v1/namespaces/{name}â)È
core_v1read the specified Namespace*readCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JX
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
û
core_v1replace the specified Namespace*replaceCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.NamespaceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
*‹
core_v1delete a Namespace*deleteCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
B¿
core_v1(partially update the specified Namespace*patchCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
J6
42"0pathname of the Namespace"name*stringòJ 
#/parameters/pretty-tJGM1-ng∂
"/api/v1/namespaces/{name}/finalizeèä
core_v1+replace finalize of the specified Namespace*replaceCoreV1NamespaceFinalize2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.NamespaceJú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
Jû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†J&$
"#/parameters/fieldManager-Qy4HdaTWJ‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J6
42"0pathname of the Namespace"name*stringòJ 
#/parameters/pretty-tJGM1-ngˇ!
 /api/v1/namespaces/{name}/status⁄!˘
core_v1&read status of the specified Namespace*readCoreV1NamespaceStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JX
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
Æ
core_v1)replace status of the specified Namespace*replaceCoreV1NamespaceStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B@
>
<bodybody *,
*#/definitions/io.k8s.api.core.v1.NamespaceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
B–
core_v12partially update status of the specified Namespace*patchCoreV1NamespaceStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJú
=
2006
4
OK.
,
*#/definitions/io.k8s.api.core.v1.Namespace
B
201;
9
Created.
,
*#/definitions/io.k8s.api.core.v1.Namespace

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
J6
42"0pathname of the Namespace"name*stringòJ 
#/parameters/pretty-tJGM1-ngà 
/api/v1/nodesˆé
core_v1"list or watch objects of kind Node*listCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJW
<
2005
3
OK-
+
)#/definitions/io.k8s.api.core.v1.NodeList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
"≥
core_v1create a Node*createCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B;
9
7bodybody *'
%#/definitions/io.k8s.api.core.v1.NodeBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J“
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node
=
2016
4
Created)
'
%#/definitions/io.k8s.api.core.v1.Node
>
2027
5
Accepted)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
*ä

core_v1delete collection of Node*deleteCoreV1CollectionNode2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
J 
#/parameters/pretty-tJGM1-ng√(
/api/v1/nodes/{name}™(’
core_v1read the specified Node*readCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JS
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
Ä
core_v1replace the specified Node*replaceCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B;
9
7bodybody *'
%#/definitions/io.k8s.api.core.v1.NodeBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jí
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node
=
2016
4
Created)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
*Õ
core_v1delete a Node*deleteCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
Bß
core_v1#partially update the specified Node*patchCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJí
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node
=
2016
4
Created)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
J1
/-"+pathname of the Node"name*stringòJ 
#/parameters/pretty-tJGM1-ng™!
/api/v1/nodes/{name}/statusä!Â
core_v1!read status of the specified Node*readCoreV1NodeStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*JS
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionget
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
ê
core_v1$replace status of the specified Node*replaceCoreV1NodeStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B;
9
7bodybody *'
%#/definitions/io.k8s.api.core.v1.NodeBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†Jí
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node
=
2016
4
Created)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionput
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
B∑
core_v1-partially update status of the specified Node*patchCoreV1NodeStatus2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJí
8
2001
/
OK)
'
%#/definitions/io.k8s.api.core.v1.Node
=
2016
4
Created)
'
%#/definitions/io.k8s.api.core.v1.Node

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
J1
/-"+pathname of the Node"name*stringòJ 
#/parameters/pretty-tJGM1-ng–
/api/v1/podsøÿ
core_v1!list or watch objects of kind Pod*listCoreV1PodForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*JV
;
2004
2
OK,
*
(#/definitions/io.k8s.api.core.v1.PodList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKﬂ
/api/v1/secretsÀ‰
core_v1$list or watch objects of kind Secret* listCoreV1SecretForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*JY
>
2007
5
OK/
-
+#/definitions/io.k8s.api.core.v1.SecretList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKá
/api/v1/serviceaccountsÎÑ
core_v1,list or watch objects of kind ServiceAccount*(listCoreV1ServiceAccountForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Ja
F
200?
=
OK7
5
3#/definitions/io.k8s.api.core.v1.ServiceAccountList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK‰
/api/v1/servicesœË
core_v1%list or watch objects of kind Service*!listCoreV1ServiceForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*JZ
?
2008
6
OK0
.
,#/definitions/io.k8s.api.core.v1.ServiceList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK€
/api/v1/watch/configmapsæ◊
core_v1uwatch individual changes to a list of ConfigMap. deprecated: use the 'watch' parameter with a list operation instead.*(watchCoreV1ConfigMapListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK⁄
/api/v1/watch/endpointsæ◊
core_v1uwatch individual changes to a list of Endpoints. deprecated: use the 'watch' parameter with a list operation instead.*(watchCoreV1EndpointsListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÀ
/api/v1/watch/events≤À
core_v1qwatch individual changes to a list of Event. deprecated: use the 'watch' parameter with a list operation instead.*$watchCoreV1EventListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÀ
/api/v1/watch/namespacesÆ«
core_v1uwatch individual changes to a list of Namespace. deprecated: use the 'watch' parameter with a list operation instead.*watchCoreV1NamespaceList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKë	
//api/v1/watch/namespaces/{namespace}/configmaps›—
core_v1uwatch individual changes to a list of ConfigMap. deprecated: use the 'watch' parameter with a list operation instead.*"watchCoreV1NamespacedConfigMapList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÑ

6/api/v1/watch/namespaces/{namespace}/configmaps/{name}…	Ö
core_v1∞watch changes to an object of kind ConfigMap. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1NamespacedConfigMap2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: ConfigMap
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ6
42"0pathname of the ConfigMap"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKê	
./api/v1/watch/namespaces/{namespace}/endpoints›—
core_v1uwatch individual changes to a list of Endpoints. deprecated: use the 'watch' parameter with a list operation instead.*"watchCoreV1NamespacedEndpointsList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÉ

5/api/v1/watch/namespaces/{namespace}/endpoints/{name}…	Ö
core_v1∞watch changes to an object of kind Endpoints. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1NamespacedEndpoints2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: Endpoints
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ6
42"0pathname of the Endpoints"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÅ	
+/api/v1/watch/namespaces/{namespace}/events—≈
core_v1qwatch individual changes to a list of Event. deprecated: use the 'watch' parameter with a list operation instead.*watchCoreV1NamespacedEventList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK	
2/api/v1/watch/namespaces/{namespace}/events/{name}π	˘
core_v1¨watch changes to an object of kind Event. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1NamespacedEvent2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jG
x-kubernetes-group-version-kind$"group: ""
kind: Event
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ2
0.",pathname of the Event"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK˘
)/api/v1/watch/namespaces/{namespace}/podsÀø
core_v1owatch individual changes to a list of Pod. deprecated: use the 'watch' parameter with a list operation instead.*watchCoreV1NamespacedPodList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÊ	
0/api/v1/watch/namespaces/{namespace}/pods/{name}±	Û
core_v1™watch changes to an object of kind Pod. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1NamespacedPod2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ0
.,"*pathname of the Pod"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÖ	
,/api/v1/watch/namespaces/{namespace}/secrets‘»
core_v1rwatch individual changes to a list of Secret. deprecated: use the 'watch' parameter with a list operation instead.*watchCoreV1NamespacedSecretList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKı	
3/api/v1/watch/namespaces/{namespace}/secrets/{name}Ω	¸
core_v1≠watch changes to an object of kind Secret. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1NamespacedSecret2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ3
1/"-pathname of the Secret"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK•	
4/api/v1/watch/namespaces/{namespace}/serviceaccountsÏ‡
core_v1zwatch individual changes to a list of ServiceAccount. deprecated: use the 'watch' parameter with a list operation instead.*'watchCoreV1NamespacedServiceAccountList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKù

;/api/v1/watch/namespaces/{namespace}/serviceaccounts/{name}›	î
core_v1µwatch changes to an object of kind ServiceAccount. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*#watchCoreV1NamespacedServiceAccount2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ;
97"5pathname of the ServiceAccount"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKâ	
-/api/v1/watch/namespaces/{namespace}/services◊À
core_v1swatch individual changes to a list of Service. deprecated: use the 'watch' parameter with a list operation instead.* watchCoreV1NamespacedServiceList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK˙	
4/api/v1/watch/namespaces/{namespace}/services/{name}¡	ˇ
core_v1Æwatch changes to an object of kind Service. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1NamespacedService2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ4
20".pathname of the Service"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKæ	
/api/v1/watch/namespaces/{name}ö	˚
core_v1∞watch changes to an object of kind Namespace. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1Namespace2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jK
x-kubernetes-group-version-kind(&group: ""
kind: Namespace
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ6
42"0pathname of the Namespace"name*stringòJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK∑
/api/v1/watch/nodesü∏
core_v1pwatch individual changes to a list of Node. deprecated: use the 'watch' parameter with a list operation instead.*watchCoreV1NodeList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK•	
/api/v1/watch/nodes/{name}Ü	Ï
core_v1´watch changes to an object of kind Node. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*watchCoreV1Node2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
jF
x-kubernetes-group-version-kind#!group: ""
kind: Node
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ1
/-"+pathname of the Node"name*stringòJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK√
/api/v1/watch/pods¨≈
core_v1owatch individual changes to a list of Pod. deprecated: use the 'watch' parameter with a list operation instead.*"watchCoreV1PodListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jE
x-kubernetes-group-version-kind" group: ""
kind: Pod
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKœ
/api/v1/watch/secretsµŒ
core_v1rwatch individual changes to a list of Secret. deprecated: use the 'watch' parameter with a list operation instead.*%watchCoreV1SecretListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jH
x-kubernetes-group-version-kind%#group: ""
kind: Secret
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÔ
/api/v1/watch/serviceaccountsÕÊ
core_v1zwatch individual changes to a list of ServiceAccount. deprecated: use the 'watch' parameter with a list operation instead.*-watchCoreV1ServiceAccountListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jP
x-kubernetes-group-version-kind-+group: ""
kind: ServiceAccount
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK”
/api/v1/watch/services∏—
core_v1swatch individual changes to a list of Service. deprecated: use the 'watch' parameter with a list operation instead.*&watchCoreV1ServiceListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
jI
x-kubernetes-group-version-kind&$group: ""
kind: Service
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÏ
/apis/·ﬁ
apisget available API versions*getAPIVersions2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJm
R
200K
I
OKC
A
?#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIGroupList

401

UnauthorizedRhttpsê
/apis/authorization.k8s.io/Ì
authorizationget information of a group*getAuthorizationAPIGroup2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJi
N
200G
E
OK?
=
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup

401

UnauthorizedRhttps†
/apis/authorization.k8s.io/v1/˝˙
authorization_v1get available resources*getAuthorizationV1APIResources2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJp
U
200N
L
OKF
D
B#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList

401

UnauthorizedRhttps‡
6/apis/authorization.k8s.io/v1/selfsubjectaccessreviews•"ÿ
authorization_v1 create a SelfSubjectAccessReview*,createAuthorizationV1SelfSubjectAccessReview2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*BW
U
Sbodybody *C
A#/definitions/io.k8s.api.authorization.v1.SelfSubjectAccessReviewJ¶
T
200M
K
OKE
C
A#/definitions/io.k8s.api.authorization.v1.SelfSubjectAccessReview
Y
201R
P
CreatedE
C
A#/definitions/io.k8s.api.authorization.v1.SelfSubjectAccessReview
Z
202S
Q
AcceptedE
C
A#/definitions/io.k8s.api.authorization.v1.SelfSubjectAccessReview

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jk
x-kubernetes-group-version-kindHFgroup: authorization.k8s.io
kind: SelfSubjectAccessReview
version: v1
Jû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†J&$
"#/parameters/fieldManager-Qy4HdaTWJ‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J 
#/parameters/pretty-tJGM1-ngÿ
5/apis/authorization.k8s.io/v1/selfsubjectrulesreviewsû"—
authorization_v1create a SelfSubjectRulesReview*+createAuthorizationV1SelfSubjectRulesReview2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*BV
T
Rbodybody *B
@#/definitions/io.k8s.api.authorization.v1.SelfSubjectRulesReviewJ£
S
200L
J
OKD
B
@#/definitions/io.k8s.api.authorization.v1.SelfSubjectRulesReview
X
201Q
O
CreatedD
B
@#/definitions/io.k8s.api.authorization.v1.SelfSubjectRulesReview
Y
202R
P
AcceptedD
B
@#/definitions/io.k8s.api.authorization.v1.SelfSubjectRulesReview

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
jj
x-kubernetes-group-version-kindGEgroup: authorization.k8s.io
kind: SelfSubjectRulesReview
version: v1
Jû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†J&$
"#/parameters/fieldManager-Qy4HdaTWJ‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J 
#/parameters/pretty-tJGM1-ngÑ
/apis/discovery.k8s.io/ËÂ
	discoveryget information of a group*getDiscoveryAPIGroup2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJi
N
200G
E
OK?
=
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup

401

UnauthorizedRhttpsî
/apis/discovery.k8s.io/v1/ıÚ
discovery_v1get available resources*getDiscoveryV1APIResources2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json:application/yaml:#application/vnd.kubernetes.protobuf:application/cborJp
U
200N
L
OKF
D
B#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList

401

UnauthorizedRhttps±
(/apis/discovery.k8s.io/v1/endpointslicesÑù
discovery_v1+list or watch objects of kind EndpointSlice*,listDiscoveryV1EndpointSliceForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Je
J
200C
A
OK;
9
7#/definitions/io.k8s.api.discovery.v1.EndpointSliceList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK›"
?/apis/discovery.k8s.io/v1/namespaces/{namespace}/endpointslicesô"Ÿ
discovery_v1+list or watch objects of kind EndpointSlice*&listDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*B-+
)#/parameters/allowWatchBookmarks-HC2hJt-JB" 
#/parameters/continue-QfD61s0iB'%
##/parameters/fieldSelector-xIcQKXFGB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCB
#/parameters/watch-XNNPZGbKJe
J
200C
A
OK;
9
7#/definitions/io.k8s.api.discovery.v1.EndpointSliceList

401

UnauthorizedRhttpsj
x-kubernetes-actionlist
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
"©
discovery_v1create an EndpointSlice*(createDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*BI
G
Ebodybody *5
3#/definitions/io.k8s.api.discovery.v1.EndpointSliceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†J¸
F
200?
=
OK7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice
K
201D
B
Created7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice
L
202E
C
Accepted7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice

401

UnauthorizedRhttpsj
x-kubernetes-actionpost
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
*«

discovery_v1"delete collection of EndpointSlice*2deleteDiscoveryV1CollectionNamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQB" 
#/parameters/continue-QfD61s0iBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B'%
##/parameters/fieldSelector-xIcQKXFGB,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB'%
##/parameters/labelSelector-5Zw57w4CB
#/parameters/limit-1NfNmdNHB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOB)'
%#/parameters/resourceVersion-5WAnf1kxB.,
*#/parameters/resourceVersionMatch-t8XhRHeCB+)
'#/parameters/sendInitialEvents-rLXlEK_kB'%
##/parameters/shardSelector-Kgyki_3_B(&
$#/parameters/timeoutSeconds-yvYezaOCJg
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj*
x-kubernetes-actiondeletecollection
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
J#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngÏ+
F/apis/discovery.k8s.io/v1/namespaces/{namespace}/endpointslices/{name}°+†
discovery_v1 read the specified EndpointSlice*&readDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*Ja
F
200?
=
OK7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice

401

UnauthorizedRhttpsj
x-kubernetes-actionget
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
Á
discovery_v1#replace the specified EndpointSlice*)replaceDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*BI
G
Ebodybody *5
3#/definitions/io.k8s.api.discovery.v1.EndpointSliceBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-Qy4HdaTWB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†JÆ
F
200?
=
OK7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice
K
201D
B
Created7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice

401

UnauthorizedRhttpsj
x-kubernetes-actionput
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
*ã
discovery_v1delete an EndpointSlice*(deleteDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:*/*B
#/parameters/body-2Y1dVQaQBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B,*
(#/parameters/gracePeriodSeconds--K5HaBOSBJH
F#/parameters/ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqjB*(
&#/parameters/orphanDependents-uRB25kX5B+)
'#/parameters/propagationPolicy-6jk3prlOJª
L
200E
C
OK=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status
R
202K
I
Accepted=
;
9#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Status

401

UnauthorizedRhttpsj 
x-kubernetes-action	delete
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
BÄ
discovery_v1,partially update the specified EndpointSlice*'patchDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor:application/json-patch+json:application/merge-patch+json:&application/strategic-merge-patch+json:application/apply-patch+yaml:application/apply-patch+cborB
#/parameters/body-78PwaGsrBû
õòïquery¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed"dryRun2string†B&$
"#/parameters/fieldManager-7c6nTn1TB‹
Ÿ÷”query≠fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered."fieldValidation2string†B
#/parameters/force-tOGGb0YiJÆ
F
200?
=
OK7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice
K
201D
B
Created7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice

401

UnauthorizedRhttpsj
x-kubernetes-actionpatch
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
J:
86"4pathname of the EndpointSlice"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngï	
./apis/discovery.k8s.io/v1/watch/endpointslices‚˚
discovery_v1ywatch individual changes to a list of EndpointSlice. deprecated: use the 'watch' parameter with a list operation instead.*1watchDiscoveryV1EndpointSliceListForAllNamespaces2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKÀ	
E/apis/discovery.k8s.io/v1/watch/namespaces/{namespace}/endpointslicesÅ	ı
discovery_v1ywatch individual changes to a list of EndpointSlice. deprecated: use the 'watch' parameter with a list operation instead.*+watchDiscoveryV1NamespacedEndpointSliceList2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj#
x-kubernetes-action
watchlist
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbK¬

L/apis/discovery.k8s.io/v1/watch/namespaces/{namespace}/endpointslices/{name}Ò	©
discovery_v1¥watch changes to an object of kind EndpointSlice. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.*'watchDiscoveryV1NamespacedEndpointSlice2application/json2application/yaml2#application/vnd.kubernetes.protobuf2application/cbor2application/json;stream=watch20application/vnd.kubernetes.protobuf;stream=watch2application/cbor-seq:*/*Jk
P
200I
G
OKA
?
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

401

UnauthorizedRhttpsj
x-kubernetes-actionwatch
j]
x-kubernetes-group-version-kind:8group: discovery.k8s.io
kind: EndpointSlice
version: v1
J-+
)#/parameters/allowWatchBookmarks-HC2hJt-JJ" 
#/parameters/continue-QfD61s0iJ'%
##/parameters/fieldSelector-xIcQKXFGJ'%
##/parameters/labelSelector-5Zw57w4CJ
#/parameters/limit-1NfNmdNHJ:
86"4pathname of the EndpointSlice"name*stringòJ#!
#/parameters/namespace-vgWSWtn3J 
#/parameters/pretty-tJGM1-ngJ)'
%#/parameters/resourceVersion-5WAnf1kxJ.,
*#/parameters/resourceVersionMatch-t8XhRHeCJ+)
'#/parameters/sendInitialEvents-rLXlEK_kJ'%
##/parameters/shardSelector-Kgyki_3_J(&
$#/parameters/timeoutSeconds-yvYezaOCJ
#/parameters/watch-XNNPZGbKJ™Ñ
ó
3io.k8s.api.authorization.v1.FieldSelectorAttributesﬂ"ìFieldSelectorAttributes indicates a field limited access. Webhook authors are encouraged to * ensure rawSelector and requirements are not both set * consider the requirements field if set * not try to parse or consider the rawSelector field if set. This is to avoid another CVE-2022-2880 (i.e. getting different systems to agree on how exactly to parse a query is not something we want), see https://www.oxeye.io/resources/golang-parameter-smuggling-attack for more details. For the *SubjectAccessReview endpoints of the kube-apiserver: * If rawSelector is empty and requirements are empty, the request is not limited. * If rawSelector is present and requirements are empty, the rawSelector will be parsed and limited if the parsing succeeds. * If rawSelector is empty and requirements are present, the requirements should be honored * If rawSelector is present and requirements are present, the request is invalid.≤
object ∫
Ø
rawSelectorü"ërawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.≤
string
Ö
requirementsÙ"Ôrequirements is the parsed interpretation of a field selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.≤
array∫O
M
K#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.FieldSelectorRequirement˙#
x-kubernetes-list-type	atomic

ó
3io.k8s.api.authorization.v1.LabelSelectorAttributesﬂ"ìLabelSelectorAttributes indicates a label limited access. Webhook authors are encouraged to * ensure rawSelector and requirements are not both set * consider the requirements field if set * not try to parse or consider the rawSelector field if set. This is to avoid another CVE-2022-2880 (i.e. getting different systems to agree on how exactly to parse a query is not something we want), see https://www.oxeye.io/resources/golang-parameter-smuggling-attack for more details. For the *SubjectAccessReview endpoints of the kube-apiserver: * If rawSelector is empty and requirements are empty, the request is not limited. * If rawSelector is present and requirements are empty, the rawSelector will be parsed and limited if the parsing succeeds. * If rawSelector is empty and requirements are present, the requirements should be honored * If rawSelector is present and requirements are present, the request is invalid.≤
object ∫
Ø
rawSelectorü"ërawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.≤
string
Ö
requirementsÙ"Ôrequirements is the parsed interpretation of a label selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.≤
array∫O
M
K#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement˙#
x-kubernetes-list-type	atomic

∞
1io.k8s.api.authorization.v1.NonResourceAttributes˙"{NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface≤
object o
8
path0"#path is the URL path of the request≤
string
3
verb+"verb is the standard HTTP verb≤
string
—
+io.k8s.api.authorization.v1.NonResourceRule°"LNonResourceRule holds information that describes a rule for the non-resourceöverbs≤
object º
Ò
nonResourceURLs›"önonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  "*" means all.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

≈
verbsª"yverbs is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

£
.io.k8s.api.authorization.v1.ResourceAttributes"tResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface≤
object Î

¬
fieldSelector∞
A#/definitions/io.k8s.api.authorization.v1.FieldSelectorAttributes"kfieldSelector describes the limitation on access based on field.  It can only limit access, not broaden it.
M
groupD"7group is the API Group of the Resource.  "*" means all.≤
string
√
labelSelector±
A#/definitions/io.k8s.api.authorization.v1.LabelSelectorAttributes"llabelSelector describes the limitation on access based on labels.  It can only limit access, not broaden it.
Ç
namez"mname is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.≤
string
Ù
	namespaceÊ"ÿnamespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview≤
string
X
resourceL"?resource is one of the existing resource types.  "*" means all.≤
string
^
subresourceO"Bsubresource is one of the existing resource types.  "" means none.≤
string
É
verb{"nverb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.≤
string
S
versionH";version is the API Version of the Resource.  "*" means all.≤
string
’	
(io.k8s.api.authorization.v1.ResourceRule®	"¨ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.överbs≤
object ‚
©
	apiGroupsõ"ÿapiGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ﬂ
resourceNamesÕ"äresourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ä
	resources¸"πresources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
 "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ƒ
verbs∫"xverbs is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic


3io.k8s.api.authorization.v1.SelfSubjectAccessReview∏"ˆSelfSubjectAccessReview checks whether or the current user can perform an action.  Not filling in a spec.namespace means "in all namespaces".  Self is a special case, because users should always be able to check whether they can perform an actionöspec≤
object µ	
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
ﬁ
metadata—
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"èmetadata is the standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
™
spec°
E#/definitions/io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpec"Xspec holds information about the request being evaluated.  user and groups must be empty
ß
statusú
C#/definitions/io.k8s.api.authorization.v1.SubjectAccessReviewStatus"Ustatus is filled in by the server and indicates whether the request is allowed or not˙q
x-kubernetes-group-version-kindNL- group: authorization.k8s.io
  kind: SelfSubjectAccessReview
  version: v1

ß
7io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpecÎ"åSelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of resourceAttributes and nonResourceAttributes must be set≤
object Õ
™
nonResourceAttributesê
?#/definitions/io.k8s.api.authorization.v1.NonResourceAttributes"MnonResourceAttributes describes information for a non-resource access request
ù
resourceAttributesÜ
<#/definitions/io.k8s.api.authorization.v1.ResourceAttributes"FresourceAttributes describes information for a resource access request
‡
2io.k8s.api.authorization.v1.SelfSubjectRulesReview©"àSelfSubjectRulesReview enumerates the set of actions the current user can perform within a namespace. The returned list of actions may be incomplete depending on the server's authorization mode, and any errors experienced during the evaluation. SelfSubjectRulesReview should be used by UIs to show/hide actions, or to quickly let an end user reason about their permissions. It should NOT Be used by external systems to drive authorization decisions as this raises confused deputy, cache lifetime/revocation, and correctness concerns. SubjectAccessReview, and LocalAccessReview are the correct way to defer authorization decisions to the API server.öspec≤
object ï	
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
ﬁ
metadata—
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"èmetadata is the standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
ä
specÅ
D#/definitions/io.k8s.api.authorization.v1.SelfSubjectRulesReviewSpec"9spec holds information about the request being evaluated.
ß
statusú
B#/definitions/io.k8s.api.authorization.v1.SubjectRulesReviewStatus"Vstatus is filled in by the server and indicates the set of actions a user can perform.˙p
x-kubernetes-group-version-kindMK- group: authorization.k8s.io
  kind: SelfSubjectRulesReview
  version: v1

·
6io.k8s.api.authorization.v1.SelfSubjectRulesReviewSpec¶"PSelfSubjectRulesReviewSpec defines the specification for SelfSubjectRulesReview.≤
object F
D
	namespace7"*namespace to evaluate rules for. Required.≤
string
ò
5io.k8s.api.authorization.v1.SubjectAccessReviewStatusﬁ"SubjectAccessReviewStatusöallowed≤
object ™
c
allowedX"Jallowed is required. True if the action would be allowed, false otherwise.≤	
boolean
Ç
denied˜"Ëdenied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.≤	
boolean
ﬁ
evaluationError "ºevaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.≤
string
]
reasonS"Freason is optional.  It indicates why a request was allowed or denied.≤
string
¬
4io.k8s.api.authorization.v1.SubjectRulesReviewStatusâ"ÁSubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.öresourceRulesönonResourceRulesö
incomplete≤
object ‡
å
evaluationError¯"ÍevaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.≤
string
›

incompleteŒ"øincomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.≤	
boolean
º
nonResourceRulesß"¥nonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.≤
array∫=
;
9#/definitions/io.k8s.api.authorization.v1.NonResourceRule˙#
x-kubernetes-list-type	atomic

Ø
resourceRulesù"≠resourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.≤
array∫:
8
6#/definitions/io.k8s.api.authorization.v1.ResourceRule˙#
x-kubernetes-list-type	atomic

˝

3io.k8s.api.core.v1.AWSElasticBlockStoreVolumeSource≈
"°Represents a Persistent Disk resource in AWS.

An AWS EBS disk must exist before mounting to a container. The disk must also be in the same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS EBS volumes support ownership management and SELinux relabeling.övolumeID≤
object á
“
fsType«"πfsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore≤
string
µ
	partitionßint32"ëpartition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).≤	
integer
¥
readOnlyß"òreadOnly value true will force the readOnly setting in VolumeMounts. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore≤	
boolean
¿
volumeID≥"•volumeID is unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore≤
string
Ÿ
io.k8s.api.core.v1.Affinityπ"1Affinity is a group of affinity scheduling rules.≤
object ˜
v
nodeAffinityf
-#/definitions/io.k8s.api.core.v1.NodeAffinity"5Describes node affinity scheduling rules for the pod.
¥
podAffinity§
,#/definitions/io.k8s.api.core.v1.PodAffinity"tDescribes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
≈
podAntiAffinity±
0#/definitions/io.k8s.api.core.v1.PodAntiAffinity"}Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
ˆ
"io.k8s.api.core.v1.AppArmorProfileœ"?AppArmorProfile defines a pod or container's AppArmor settings.ötype≤
object É
Å
localhostProfileÏ"ﬁlocalhostProfile indicates a profile loaded on the node that should be used. The profile must be preconfigured on the node to work. Must match the loaded name of the profile. Must be set if and only if type is "Localhost".≤
string
¸
typeÛ"Âtype indicates which kind of AppArmor profile will be applied. Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.≤
string˙r
x-kubernetes-unions[Y- discriminator: type
  fields-to-discriminateBy:
    localhostProfile: LocalhostProfile

ò
!io.k8s.api.core.v1.AttachedVolumeÚ"4AttachedVolume describes a volume attached to a nodeönameö
devicePath≤
object ô
e

devicePathW"JDevicePath represents the device path where the volume should be available≤
string
0
name("Name of the attached volume≤
string
Â
(io.k8s.api.core.v1.AzureDiskVolumeSource∏"TAzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.ödiskNameödiskURI≤
object æ
^
cachingModeO"BcachingMode is the Host Caching mode: None, Read Only, Read Write.≤
string
R
diskNameF"9diskName is the Name of the data disk in the blob storage≤
string
K
diskURI@"3diskURI is the URI of data disk in the blob storage≤
string
 
fsTypeø"±fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.≤
string
È
kind‡"“kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared≤
string
Å
readOnlyu"greadOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
ﬁ
(io.k8s.api.core.v1.AzureFileVolumeSource±"WAzureFile represents an Azure File Service mount on the host and bind mount to the pod.ö
secretNameö	shareName≤
object ∞
Å
readOnlyu"greadOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
m

secretName_"RsecretName is the  name of secret that contains Azure Storage Account Name and Key≤
string
;
	shareName."!shareName is the azure share Name≤
string
µ	
io.k8s.api.core.v1.Bindingñ	"YBinding ties one object to another; for example, a pod is bound to a node by a scheduler.ötarget≤
object —
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
}
targets
0#/definitions/io.k8s.api.core.v1.ObjectReference"?The target object that you want to bind to the standard object.˙O
x-kubernetes-group-version-kind,*- group: ""
  kind: Binding
  version: v1

‰	
"io.k8s.api.core.v1.CSIVolumeSourceΩ	"TRepresents a source location of a volume to mount, managed by an external CSI driverödriver≤
object œ
¢
driveró"âdriver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.≤
string
¬
fsType∑"©fsType to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.≤
string
•
nodePublishSecretRefå
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"“nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
v
readOnlyj"\readOnly specifies a read-only configuration for the volume. Defaults to false (read/write).≤	
boolean
¬
volumeAttributes≠"èvolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.™
≤
string≤
object
Æ
io.k8s.api.core.v1.Capabilitiesä"<Adds and removes POSIX capabilities from running containers.≤
object Ω
[
addT"Added capabilities≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

^
dropV"Removed capabilities≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

È

%io.k8s.api.core.v1.CephFSVolumeSourceø
"çRepresents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.ömonitors≤
object ï	
Ÿ
monitorsÃ"âmonitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

m
pathe"Xpath is Optional: Used as the mounted root, rather than the full Ceph tree, default is /≤
string
⁄
readOnlyÕ"æreadOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it≤	
boolean
Ã

secretFileΩ"ØsecretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it≤
string
¯
	secretRefÍ
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"∞secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
†
useró"âuser is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it≤
string
ø
%io.k8s.api.core.v1.CinderVolumeSourceï"ÎRepresents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling.övolumeID≤
object ç
ë
fsTypeÜ"¯fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md≤
string
¿
readOnly≥"§readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md≤	
boolean
´
	secretRefù
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"dsecretRef is optional: points to a secret object containing parameters used to connect to OpenStack.
Ö
volumeIDy"lvolumeID used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md≤
string

!io.k8s.api.core.v1.ClientIPConfig "QClientIPConfig represents the configurations of Client IP based session affinity.≤
object Ë
Â
timeoutSeconds“int32"ºtimeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be >0 && <=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).≤	
integer
È
/io.k8s.api.core.v1.ClusterTrustBundleProjectionµ"åClusterTrustBundleProjection describes how to select a set of ClusterTrustBundle objects and project their contents into the pod filesystem.öpath≤
object ê

º
labelSelector™
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector"ÂSelect all ClusterTrustBundles that match this label selector.  Only has effect if signerName is set.  Mutually-exclusive with name.  If unset, interpreted as "match nothing".  If set but empty, interpreted as "match everything".
~
namev"iSelect a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector.≤
string
∫
optional≠"ûIf true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles.≤	
boolean
L
pathD"7Relative path from the volume root to write the bundle.≤
string
 

signerNameª"≠Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated.≤
string
ˆ
userÌint64"◊user is Optional: The owner UID of the created file. If specified, the item-level user field takes precedence over defaultUser. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
˙
io.k8s.api.core.v1.ConfigMapŸ"7ConfigMap holds configuration data for pods to consume.≤
object Ω
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
´

binaryDataú"¯BinaryData contains the binary data. Each key must consist of alphanumeric characters, '-', '_' or '.'. BinaryData can contain byte sequences that are not in the UTF-8 range. The keys stored in BinaryData must not overlap with the ones in the Data field, this is enforced during validation process. Using this field will require 1.10+ apiserver and kubelet. Note: BinaryData keys are not currently propagated to container env vars via ConfigMapKeyRef or ConfigMapRef env sources; only Data keys are used.™
byte≤
string≤
object
œ
data∆"®Data contains the configuration data. Each key must consist of alphanumeric characters, '-', '_' or '.'. Values with non-UTF-8 byte sequences must use the BinaryData field. The keys stored in Data must not overlap with the keys in the BinaryData field, this is enforced during validation process.™
≤
string≤
object
Ë
	immutable⁄"ÀImmutable, if set to true, ensures that data stored in the ConfigMap cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.≤	
boolean
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata˙Q
x-kubernetes-group-version-kind.,- group: ""
  kind: ConfigMap
  version: v1

Ã
%io.k8s.api.core.v1.ConfigMapEnvSource¢"ìConfigMapEnvSource selects a ConfigMap to populate the environment variables with.

The contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables. Keys in the BinaryData field are not currently propagated to container env vars.≤
object ˝
±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
G
optional;"-Specify whether the ConfigMap must be defined≤	
boolean
´
'io.k8s.api.core.v1.ConfigMapKeySelectorˇ"Selects a key from a ConfigMap.ökey≤
object §
ô
keyë"ÉThe key to select from the ConfigMap's Data field. Keys in the BinaryData field are not currently propagated to container env vars.≤
string
±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
R
optionalF"8Specify whether the ConfigMap or its key must be defined≤	
boolean˙"
x-kubernetes-map-type	atomic

Ù
 io.k8s.api.core.v1.ConfigMapListœ"CConfigMapList is a resource containing a list of ConfigMap objects.öitems≤
object õ
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
f
items]" Items is the list of ConfigMaps.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.ConfigMap
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
≥
metadata¶
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"gMore info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata˙U
x-kubernetes-group-version-kind20- group: ""
  kind: ConfigMapList
  version: v1

∫
,io.k8s.api.core.v1.ConfigMapNodeConfigSourceâ"ﬂConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configurationö	namespaceönameökubeletConfigKey≤
object Ú
π
kubeletConfigKey§"ñKubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.≤
string
p
nameh"[Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.≤
string

	namespacer"eNamespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.≤
string
±
resourceVersionù"èResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.≤
string
å
uidÑ"wUID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.≤
string
ç
&io.k8s.api.core.v1.ConfigMapProjection‚
"ÂAdapts a ConfigMap into a projected volume.

The contents of the target ConfigMap's Data field will be presented in a projected volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. Note that this is identical to a configmap volume source without the default mode.≤
object Î
÷
itemsÃ"Ëitems if unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.KeyToPath˙#
x-kubernetes-list-type	atomic

±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
\
optionalP"Boptional specify whether the ConfigMap or its keys must be defined≤	
boolean
ç
(io.k8s.api.core.v1.ConfigMapVolumeSource‡"≈Adapts a ConfigMap into a volume.

The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. ConfigMap volumes support ownership management and SELinux relabeling.≤
object â
˘
defaultModeÈint32"”defaultMode is optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.≤	
integer
ü
defaultUserèint64"˘defaultUser is Optional: The owner UID of the created files by default. The defaultUser field is only used as a fallback when the item-level user field is unset. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
÷
itemsÃ"Ëitems if unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.KeyToPath˙#
x-kubernetes-list-type	atomic

±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
\
optionalP"Boptional specify whether the ConfigMap or its keys must be defined≤	
boolean
ÛP
io.k8s.api.core.v1.Container“P"AA single application container that you want to run within a pod.öname≤
object ˘O
ÿ
argsœ"åArguments to the entrypoint. The container image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

Ù
commandË"•Entrypoint array. Not executed within a shell. The container image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

¨
env§"IList of environment variables to set in the container. Cannot be updated.≤
array∫+
)
'#/definitions/io.k8s.api.core.v1.EnvVar˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

–
envFromƒ"‹List of sources to populate environment variables in the container. The keys defined within a source may consist of any printable ASCII characters except '='. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.≤
array∫2
0
.#/definitions/io.k8s.api.core.v1.EnvFromSource˙#
x-kubernetes-list-type	atomic

ê
imageÜ"¯Container image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.≤
string
á
imagePullPolicyÛ"ÂImage pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images≤
string
®
	lifecycleö
*#/definitions/io.k8s.api.core.v1.Lifecycle"lActions that the management system should take in response to container lifecycle events. Cannot be updated.
Ç
livenessProbe
&#/definitions/io.k8s.api.core.v1.Probe"≈Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
ï
nameå"Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.≤
string
û
portsî"îList of ports to expose from the container. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Modifying this array with strategic merge patch may corrupt the data. For more information See https://github.com/kubernetes/kubernetes/issues/108255. Cannot be updated.≤
array∫2
0
.#/definitions/io.k8s.api.core.v1.ContainerPort˙;
x-kubernetes-list-map-keys- containerPort
- protocol
˙ 
x-kubernetes-list-typemap
˙0
x-kubernetes-patch-merge-keycontainerPort
˙'
x-kubernetes-patch-strategymerge

°
readinessProbeé
&#/definitions/io.k8s.api.core.v1.Probe"„Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
‹
resizePolicyÀ"\Resources resize policy for the container. This field cannot be set on ephemeral containers.≤
array∫:
8
6#/definitions/io.k8s.api.core.v1.ContainerResizePolicy˙#
x-kubernetes-list-type	atomic

‚
	resources‘
5#/definitions/io.k8s.api.core.v1.ResourceRequirements"öCompute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
Ô
restartPolicy›"œRestartPolicy defines the restart behavior of individual containers in a pod. This overrides the pod-level restart policy. When this field is not specified, the restart behavior is defined by the Pod's restart policy and the container type. Additionally, setting the RestartPolicy as "Always" for the init container will have the following effect: this init container will be continually restarted on exit until all regular containers have terminated. Once all regular containers have completed, all init containers with restartPolicy "Always" will be shut down. This lifecycle differs from normal init containers and is often referred to as a "sidecar" container. Although this init container still starts in the init container sequence, it does not wait for the container to complete before proceeding to the next init container. Instead, the next init container starts immediately after this init container is started, or after any startupProbe has successfully completed.≤
string
Ï
restartPolicyRules’"ÊRepresents a list of rules to be checked to determine if the container should be restarted on exit. The rules are evaluated in order. Once a rule matches a container exit condition, the remaining rules are ignored. If no rule matches the container exit condition, the Container-level restart policy determines the whether the container is restarted or not. Constraints on the rules: - At most 20 rules are allowed. - Rules can have the same action. - Identical rules are not forbidden in validations. When rules are specified, container MUST set RestartPolicy explicitly even it if matches the Pod's RestartPolicy.≤
array∫9
7
5#/definitions/io.k8s.api.core.v1.ContainerRestartRule˙#
x-kubernetes-list-type	atomic

…
securityContextµ
0#/definitions/io.k8s.api.core.v1.SecurityContext"ÄSecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
‘
startupProbe√
&#/definitions/io.k8s.api.core.v1.Probe"òStartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
œ
stdin≈"∂Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.≤	
boolean
◊
	stdinOnce…"∫Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false≤	
boolean
≈
terminationMessagePath™"úOptional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.≤
string
Ê
terminationMessagePolicy…"ªIndicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.≤
string
Å
ttyz"lWhether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.≤	
boolean
∆
volumeDevices¥"GvolumeDevices is the list of block devices to be used by the container.≤
array∫1
/
-#/definitions/io.k8s.api.core.v1.VolumeDevice˙-
x-kubernetes-list-map-keys- devicePath
˙ 
x-kubernetes-list-typemap
˙-
x-kubernetes-patch-merge-keydevicePath
˙'
x-kubernetes-patch-strategymerge

√
volumeMounts≤"HPod volumes to mount into the container's filesystem. Cannot be updated.≤
array∫0
.
,#/definitions/io.k8s.api.core.v1.VolumeMount˙,
x-kubernetes-list-map-keys- mountPath
˙ 
x-kubernetes-list-typemap
˙,
x-kubernetes-patch-merge-key
mountPath
˙'
x-kubernetes-patch-strategymerge

¿

workingDir±"£Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.≤
string
•
3io.k8s.api.core.v1.ContainerExtendedResourceRequestÌ"vContainerExtendedResourceRequest has the mapping of container name, extended resource name to the device request name.öcontainerNameöresourceNameörequestName≤
object π
M
containerName<"/The name of the container requesting resources.≤
string
|
requestNamem"`The name of the request in the special ResourceClaim which corresponds to the extended resource.≤
string
j
resourceNameZ"MThe name of the extended resource in that container which gets backed by DRA.≤
string
Û
!io.k8s.api.core.v1.ContainerImageÕ"Describe a container image≤
object ¢
‹
names“"èNames by which this image is known. e.g. ["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"]≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

A
	sizeBytes4int64"The size of the image in bytes.≤	
integer
ﬁ
 io.k8s.api.core.v1.ContainerPortπ">ContainerPort represents a network port in a single container.öcontainerPort≤
object ⁄
à
containerPortwint32"bNumber of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.≤	
integer
A
hostIP7"*What host IP to bind the external port to.≤
string
Â
hostPortÿint32"¬Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.≤	
integer
∆
nameΩ"ØIf specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.≤
string
Y
protocolM"@Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".≤
string
±
(io.k8s.api.core.v1.ContainerResizePolicyÑ"JContainerResizePolicy represents resource resize policy for the container.öresourceNameörestartPolicy≤
object ä
~
resourceNamen"aName of the resource to which this resource resize policy applies. Supported values: cpu, memory.≤
string
á
restartPolicyv"iRestart policy to apply when specified resource is resized. If not specified, it defaults to NotRequired.≤
string
ª
'io.k8s.api.core.v1.ContainerRestartRuleè"?ContainerRestartRule describes how a container exit is handled.öaction≤
object ∂
©
actionû"êSpecifies the action taken on a container exit if the requirements are satisfied. The only possible value is "Restart" to restart the container.≤
string
á
	exitCodesz
@#/definitions/io.k8s.api.core.v1.ContainerRestartRuleOnExitCodes"6Represents the exit codes to check on container exits.
—
2io.k8s.api.core.v1.ContainerRestartRuleOnExitCodesö"qContainerRestartRuleOnExitCodes describes the condition for handling an exited container based on its exit codes.öoperator≤
object ç
’
operator»"∫Represents the relationship between the container exit code(s) and the specified values. Possible values are: - In: the requirement is satisfied if the container exit code is in the
  set of specified values.
- NotIn: the requirement is satisfied if the container exit code is
  not in the set of specified values.≤
string
≤
valuesß"`Specifies the set of values to check for container exit codes. At most 255 elements are allowed.≤
array∫
int32≤	
integer˙ 
x-kubernetes-list-typeset

û
!io.k8s.api.core.v1.ContainerState¯"•ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.≤
object ¡
f
running[
6#/definitions/io.k8s.api.core.v1.ContainerStateRunning"!Details about a running container
o

terminateda
9#/definitions/io.k8s.api.core.v1.ContainerStateTerminated"$Details about a terminated container
f
waiting[
6#/definitions/io.k8s.api.core.v1.ContainerStateWaiting"!Details about a waiting container

(io.k8s.api.core.v1.ContainerStateRunning√"8ContainerStateRunning is a running state of a container.≤
object {
y
	startedAtl
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"1Time at which the container was last (re-)started
¶
+io.k8s.api.core.v1.ContainerStateTerminatedˆ">ContainerStateTerminated is a terminated state of a container.öexitCode≤
object ú
R
containerIDC"6Container's ID in the format '<type>://<container_id>'≤
string
W
exitCodeKint32"6Exit status from the last termination of the container≤	
integer
t

finishedAtf
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"+Time at which the container last terminated
O
messageD"7Message regarding the last termination of the container≤
string
P
reasonF"9(brief) reason from the last termination of the container≤
string
P
signalFint32"1Signal from the last termination of the container≤	
integer
Å
	startedAtt
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"9Time at which previous execution of the container started
ê
(io.k8s.api.core.v1.ContainerStateWaiting„"8ContainerStateWaiting is a waiting state of a container.≤
object ö
O
messageD"7Message regarding why the container is not yet running.≤
string
G
reason="0(brief) reason the container is not yet running.≤
string
• 
"io.k8s.api.core.v1.ContainerStatus˛"JContainerStatus contains details for the current status of this container.önameöreadyörestartCountöimageöimageID≤
object Û
œ
allocatedResources∏"ËAllocatedResources represents the compute resources allocated for this container by the node. Kubelet sets this value to Container.Resources.Requests upon successful pod admission and after successfully admitting desired pod resize.™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
€
allocatedResourcesStatusæ"[AllocatedResourcesStatus represents the status of various resources allocated for this Pod.≤
array∫3
1
/#/definitions/io.k8s.api.core.v1.ResourceStatus˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

›
containerIDÕ"øContainerID is the ID of the container in the format '<type>://<container_id>'. Where type is a container runtime identifier, returned from Version call of CRI API (for example "containerd").≤
string
ã
imageÅ"ÛImage is the name of container image that the container is running. The container image may not match the image used in the PodSpec, as it may have been resolved by the runtime. More info: https://kubernetes.io/docs/concepts/containers/images.≤
string
¬
imageID∂"®ImageID is the image ID of the container's image. The image ID may not match the image ID of the image used in the PodSpec, as it may have been resolved by the runtime.≤
string
ã
	lastState˝
/#/definitions/io.k8s.api.core.v1.ContainerState"…LastTerminationState holds the last termination state of the container to help debug container crashes and restarts. This field is not populated if the container is still running and RestartCount is 0.
∏
nameØ"°Name is a DNS_LABEL representing the unique name of the container. Each container in a pod must have a unique name across all container types. Cannot be updated.≤
string
˜
readyÌ"ﬁReady specifies whether the container is currently passing its readiness check. The value will change as readiness probes keep executing. If no readiness probes are specified, this field defaults to true once the container is fully started (see Started field).

The value is typically used to determine whether a container is ready to accept traffic.≤	
boolean
˛
	resources
5#/definitions/io.k8s.api.core.v1.ResourceRequirements"∂Resources represents the compute resource requests and limits that have been successfully enacted on the running container after it has been started or has been successfully resized.
≠
restartCountúint32"ÜRestartCount holds the number of times the container has been restarted. Kubelet makes an effort to always increment the value, but there are cases when the state may be lost due to node restarts and then the value may be reset to 0. The value is never negative.≤	
integer
á
started˚"ÏStarted indicates whether the container has finished its postStart lifecycle hook and passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. In both cases, startup probes will run again. Is always true when no startupProbe is defined and container is running and has passed the postStart lifecycle hook. The null value must be treated the same as false.≤	
boolean
x
stateo
/#/definitions/io.k8s.api.core.v1.ContainerState"<State holds details about the container's current condition.
Z

stopSignalL"?StopSignal reports the effective stop signal for this container≤
string
ù
userî
.#/definitions/io.k8s.api.core.v1.ContainerUser"bUser represents user identity information initially attached to the first process of the container
ô
volumeMountsà"Status of volume mounts.≤
array∫6
4
2#/definitions/io.k8s.api.core.v1.VolumeMountStatus˙,
x-kubernetes-list-map-keys- mountPath
˙ 
x-kubernetes-list-typemap
˙,
x-kubernetes-patch-merge-key
mountPath
˙'
x-kubernetes-patch-strategymerge

˘
 io.k8s.api.core.v1.ContainerUser‘"2ContainerUser represents user identity information≤
object ë
é
linuxÑ
3#/definitions/io.k8s.api.core.v1.LinuxContainerUser"ÃLinux holds user identity information initially attached to the first process of the containers in Linux. Note that the actual running identity can be changed if the process has enough privilege to do so.
¡
!io.k8s.api.core.v1.DaemonEndpointõ"CDaemonEndpoint contains information about a single Daemon endpoint.öPort≤
object A
?
Port7int32""Port number of the given endpoint.≤	
integer
˝
(io.k8s.api.core.v1.DownwardAPIProjection–"ôRepresents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.≤
object •
¢
itemsò")Items is a list of DownwardAPIVolume file≤
array∫:
8
6#/definitions/io.k8s.api.core.v1.DownwardAPIVolumeFile˙#
x-kubernetes-list-type	atomic

‘
(io.k8s.api.core.v1.DownwardAPIVolumeFileß"XDownwardAPIVolumeFile represents information to create the file containing the pod fieldöpath≤
object ∑

´
fieldRefû
4#/definitions/io.k8s.api.core.v1.ObjectFieldSelector"fRequired: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
ª
mode≤int32"úOptional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.≤	
integer
ﬁ
path’"«Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'≤
string
Ó
resourceFieldRefŸ
6#/definitions/io.k8s.api.core.v1.ResourceFieldSelector"ûSelects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
ˆ
userÌint64"◊user is Optional: The owner UID of the created file. If specified, the item-level user field takes precedence over defaultUser. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
Ã	
*io.k8s.api.core.v1.DownwardAPIVolumeSourceù	"ìDownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.≤
object ¯
¨
defaultModeúint32"ÜOptional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.≤	
integer
ü
defaultUserèint64"˘defaultUser is Optional: The owner UID of the created files by default. The defaultUser field is only used as a fallback when the item-level user field is unset. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
§
itemsö"+Items is a list of downward API volume file≤
array∫:
8
6#/definitions/io.k8s.api.core.v1.DownwardAPIVolumeFile˙#
x-kubernetes-list-type	atomic

√
'io.k8s.api.core.v1.EmptyDirVolumeSourceó"uRepresents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.≤
object ë

ó
mediumå"˛medium represents what type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir≤
string
Ù
modeÎint32"’mode specifies the permission bits for the emptyDir directory, in numeric notation (e.g., 0755, 01777). Must be a value between 0000 and 01777. If not specified, defaults to 0777. This might be in conflict with other options that affect the file mode, like fsGroup. If fsGroup is specified, the fsGroup permissions will override the mode specified here. This field has no effect on Windows. This field is alpha and requires EmptyDirVolumeMode featuregate to be enabled.≤	
integer
˝
	sizeLimitÔ
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity"ØsizeLimit is the total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
§
"io.k8s.api.core.v1.EndpointAddress˝"jEndpointAddress is a tuple that describes single IP address. Deprecated: This API is deprecated in v1.33+.öip≤
object ÿ
6
hostname*"The Hostname of this endpoint≤
string
∂
ipØ"°The IP of this endpoint. May not be loopback (127.0.0.0/8 or ::1), link-local (169.254.0.0/16 or fe80::/10), or link-local multicast (224.0.0.0/24 or ff02::/16).≤
string
w
nodeNamek"^Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.≤
string
l
	targetRef_
0#/definitions/io.k8s.api.core.v1.ObjectReference"+Reference to object providing the endpoint.˙"
x-kubernetes-map-type	atomic

¢
io.k8s.api.core.v1.EndpointPort˛
"cEndpointPort is a tuple that describes a single port. Deprecated: This API is deprecated in v1.33+.öport≤
object ﬁ	
â
appProtocol˘"ÎThe application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:

* Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and https://www.iana.org/assignments/service-names).

* Kubernetes-defined prefixed names:
  * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
  * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
  * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455

* Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.≤
string
¨
name£"ïThe name of this port.  This must match the 'name' field in the corresponding ServicePort. Must be a DNS_LABEL. Optional only if one port is defined.≤
string
=
port5int32" The port number of the endpoint.≤	
integer
b
protocolV"IThe IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.≤
string˙"
x-kubernetes-map-type	atomic

ü

!io.k8s.api.core.v1.EndpointSubset˘	"€EndpointSubset is a group of addresses with a common set of ports. The expanded set of endpoints is the Cartesian product of Addresses x Ports. For example, given:

	{
	  Addresses: [{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}],
	  Ports:     [{"name": "a", "port": 8675}, {"name": "b", "port": 309}]
	}

The resulting set of endpoints can be viewed as:

	a: [ 10.10.1.1:8675, 10.10.2.2:8675 ],
	b: [ 10.10.1.1:309, 10.10.2.2:309 ]

Deprecated: This API is deprecated in v1.33+.≤
object å
ë
	addressesÉ"ôIP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.EndpointAddress˙#
x-kubernetes-list-type	atomic

œ
notReadyAddressesπ"œIP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.EndpointAddress˙#
x-kubernetes-list-type	atomic

£
portsô"3Port numbers available on the related IP addresses.≤
array∫1
/
-#/definitions/io.k8s.api.core.v1.EndpointPort˙#
x-kubernetes-list-type	atomic

§
io.k8s.api.core.v1.EndpointsÉ"ÙEndpoints is a collection of endpoints that implement the actual service. Example:

	 Name: "mysvc",
	 Subsets: [
	   {
	     Addresses: [{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}],
	     Ports: [{"name": "a", "port": 8675}, {"name": "b", "port": 309}]
	   },
	   {
	     Addresses: [{"ip": "10.10.3.3"}],
	     Ports: [{"name": "a", "port": 93}, {"name": "b", "port": 76}]
	   },
	]

Endpoints is a legacy API and does not contain information about all Service features. Use discoveryv1.EndpointSlice for complete information about Service endpoints.

Deprecated: This API is deprecated in v1.33+. Use discoveryv1.EndpointSlice.≤
object ©
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
‘
subsets»"ﬂThe set of all endpoints is the union of all subsets. Addresses are placed into subsets according to the IPs they share. A single address with multiple ports, some of which are ready and some of which are not (because they come from different containers) will result in the address being displayed in different subsets for the different ports. No address will appear in both Addresses and NotReadyAddresses in the same subset. Sets of addresses and ports that comprise a service.≤
array∫3
1
/#/definitions/io.k8s.api.core.v1.EndpointSubset˙#
x-kubernetes-list-type	atomic
˙Q
x-kubernetes-group-version-kind.,- group: ""
  kind: Endpoints
  version: v1

í	
 io.k8s.api.core.v1.EndpointsListÌ"SEndpointsList is a list of endpoints. Deprecated: This API is deprecated in v1.33+.öitems≤
object ©
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
X
itemsO"List of endpoints.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.Endpoints
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙U
x-kubernetes-group-version-kind20- group: ""
  kind: EndpointsList
  version: v1

”
 io.k8s.api.core.v1.EnvFromSourceÆ"EEnvFromSource represents the source of a set of ConfigMaps or Secrets≤
object ÿ
c
configMapRefS
3#/definitions/io.k8s.api.core.v1.ConfigMapEnvSource"The ConfigMap to select from
î
prefixâ"|Optional text to prepend to the name of each environment variable. May consist of any printable ASCII characters except '='.≤
string
Z
	secretRefM
0#/definitions/io.k8s.api.core.v1.SecretEnvSource"The Secret to select from
ç
io.k8s.api.core.v1.EnvVarÔ"AEnvVar represents an environment variable present in a Container.öname≤
object ñ
p
nameh"[Name of the environment variable. May consist of any printable ASCII characters except '='.≤
string
ç
valueÉ"ıVariable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".≤
string
ë
	valueFromÉ
-#/definitions/io.k8s.api.core.v1.EnvVarSource"RSource for the environment variable's value. Cannot be used if value is not empty.
Ω
io.k8s.api.core.v1.EnvVarSourceô"<EnvVarSource represents a source for the value of an EnvVar.≤
object Ã
i
configMapKeyRefV
5#/definitions/io.k8s.api.core.v1.ConfigMapKeySelector"Selects a key of a ConfigMap.
û
fieldRefë
4#/definitions/io.k8s.api.core.v1.ObjectFieldSelector"ÿSelects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
û

fileKeyRefè
0#/definitions/io.k8s.api.core.v1.FileKeySelector"[FileKeyRef selects a key of the env file. Requires the EnvFiles feature gate to be enabled.
§
resourceFieldRefè
6#/definitions/io.k8s.api.core.v1.ResourceFieldSelector"‘Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
v
secretKeyReff
2#/definitions/io.k8s.api.core.v1.SecretKeySelector"0Selects a key of a secret in the pod's namespace
ΩB
%io.k8s.api.core.v1.EphemeralContainerìB"íAn EphemeralContainer is a temporary container that you may add to an existing Pod for user-initiated activities such as debugging. Ephemeral containers have no resource or scheduling guarantees, and they will not be restarted when they exit or when a Pod is removed or restarted. The kubelet may evict a Pod if an ephemeral container causes the Pod to exceed its resource allocation.

To add an ephemeral container, use the ephemeralcontainers subresource of an existing Pod. Ephemeral containers may not be removed or restarted.öname≤
object Ë=
Œ
args≈"ÇArguments to the entrypoint. The image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

Í
commandﬁ"õEntrypoint array. Not executed within a shell. The image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

¨
env§"IList of environment variables to set in the container. Cannot be updated.≤
array∫+
)
'#/definitions/io.k8s.api.core.v1.EnvVar˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

–
envFromƒ"‹List of sources to populate environment variables in the container. The keys defined within a source may consist of any printable ASCII characters except '='. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.≤
array∫2
0
.#/definitions/io.k8s.api.core.v1.EnvFromSource˙#
x-kubernetes-list-type	atomic

l
imagec"VContainer image name. More info: https://kubernetes.io/docs/concepts/containers/images≤
string
á
imagePullPolicyÛ"ÂImage pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images≤
string
m
	lifecycle`
*#/definitions/io.k8s.api.core.v1.Lifecycle"2Lifecycle is not allowed for ephemeral containers.
k
livenessProbeZ
&#/definitions/io.k8s.api.core.v1.Probe"0Probes are not allowed for ephemeral containers.
©
name†"íName of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.≤
string
∏
portsÆ"/Ports are not allowed for ephemeral containers.≤
array∫2
0
.#/definitions/io.k8s.api.core.v1.ContainerPort˙;
x-kubernetes-list-map-keys- containerPort
- protocol
˙ 
x-kubernetes-list-typemap
˙0
x-kubernetes-patch-merge-keycontainerPort
˙'
x-kubernetes-patch-strategymerge

l
readinessProbeZ
&#/definitions/io.k8s.api.core.v1.Probe"0Probes are not allowed for ephemeral containers.
™
resizePolicyô"*Resources resize policy for the container.≤
array∫:
8
6#/definitions/io.k8s.api.core.v1.ContainerResizePolicy˙#
x-kubernetes-list-type	atomic

¡
	resources≥
5#/definitions/io.k8s.api.core.v1.ResourceRequirements"zResources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
≤
restartPolicy†"íRestart policy for the container to manage the restart behavior of each container within a pod. You cannot set this field on ephemeral containers.≤
string
ú
restartPolicyRulesÖ"ñRepresents a list of rules to be checked to determine if the container should be restarted on exit. You cannot set this field on ephemeral containers.≤
array∫9
7
5#/definitions/io.k8s.api.core.v1.ContainerRestartRule˙#
x-kubernetes-list-type	atomic

á
securityContextÛ
0#/definitions/io.k8s.api.core.v1.SecurityContext"æOptional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
j
startupProbeZ
&#/definitions/io.k8s.api.core.v1.Probe"0Probes are not allowed for ephemeral containers.
œ
stdin≈"∂Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.≤	
boolean
◊
	stdinOnce…"∫Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false≤	
boolean
‘
targetContainerNameº"ÆIf set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container uses the namespaces configured in the Pod spec.

The container runtime must implement support for this feature. If the runtime does not support namespace targeting then the result of setting this field is undefined.≤
string
≈
terminationMessagePath™"úOptional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.≤
string
Ê
terminationMessagePolicy…"ªIndicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.≤
string
Å
ttyz"lWhether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.≤	
boolean
∆
volumeDevices¥"GvolumeDevices is the list of block devices to be used by the container.≤
array∫1
/
-#/definitions/io.k8s.api.core.v1.VolumeDevice˙-
x-kubernetes-list-map-keys- devicePath
˙ 
x-kubernetes-list-typemap
˙-
x-kubernetes-patch-merge-keydevicePath
˙'
x-kubernetes-patch-strategymerge

˝
volumeMountsÏ"ÅPod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers. Cannot be updated.≤
array∫0
.
,#/definitions/io.k8s.api.core.v1.VolumeMount˙,
x-kubernetes-list-map-keys- mountPath
˙ 
x-kubernetes-list-typemap
˙,
x-kubernetes-patch-merge-key
mountPath
˙'
x-kubernetes-patch-strategymerge

¿

workingDir±"£Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.≤
string
Ë	
(io.k8s.api.core.v1.EphemeralVolumeSourceª	"JRepresents an ephemeral volume that is handled by a normal storage driver.≤
object ‡
›
volumeClaimTemplate≈
>#/definitions/io.k8s.api.core.v1.PersistentVolumeClaimTemplate"ÇWill be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).

An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.

This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.

Required, must not be nil.
Ù
io.k8s.api.core.v1.Event◊"âEvent is a report of an event somewhere in the cluster.  Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.ömetadataöinvolvedObject≤
object –
V
actionL"?What action was taken/failed regarding to the Regarding object.≤
string
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
J
countAint32",The number of times this event has occurred.≤	
integer
u
	eventTimeh
<#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime"(Time when this Event was first observed.
¶
firstTimestampì
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"XThe time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
j
involvedObjectX
0#/definitions/io.k8s.api.core.v1.ObjectReference"$The object that this event is about.
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
ï
lastTimestampÉ
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"HThe time at which the most recent occurrence of this event was recorded.
U
messageJ"=A human-readable description of the status of this operation.≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
ô
reasoné"ÄThis should be a short, machine understandable string that gives the reason for the transition into the object's current status.≤
string
r
relatedg
0#/definitions/io.k8s.api.core.v1.ObjectReference"3Optional secondary object for more complex actions.
p
reportingComponentZ"MName of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.≤
string
U
reportingInstance@"3ID of the controller instance, e.g. `kubelet-xyzf`.≤
string
é
seriesÉ
,#/definitions/io.k8s.api.core.v1.EventSeries"SData about the Event series this event represents or nil if it's a singleton Event.
è
sourceÑ
,#/definitions/io.k8s.api.core.v1.EventSource"TThe component reporting this event. Should be a short machine understandable string.
a
typeY"LType of this event (Normal, Warning), new types could be added in the future≤
string˙M
x-kubernetes-group-version-kind*(- group: ""
  kind: Event
  version: v1

Õ
io.k8s.api.core.v1.EventList¨"EventList is a list of events.öitems≤
object °
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
P
itemsG"List of events≤
array∫*
(
&#/definitions/io.k8s.api.core.v1.Event
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙Q
x-kubernetes-group-version-kind.,- group: ""
  kind: EventList
  version: v1

Å
io.k8s.api.core.v1.EventSeriesﬁ"qEventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time.≤
object ‹
`
countWint32"BNumber of occurrences in this series up to the last heartbeat time≤	
integer
x
lastObservedTimed
<#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime"$Time of the last occurrence observed
Î
io.k8s.api.core.v1.EventSource»".EventSource contains information for an event.≤
object â
F
	component9",Component from which the event is generated.≤
string
?
host7"*Node name on which the event is generated.≤
string
„

$io.k8s.api.core.v1.EvictionResponder∫
"ÊEvictionResponder allows you to specify the responder reacting to an Eviction. Responders should observe and communicate through the Eviction Resource API to help with the graceful eviction of a target (e.g. termination of a pod).önameöpriority≤
object ã
õ
nameí"Ñname allows you to identify the responder responding to the Eviction.

It must be a valid domain-prefixed key (such as "acme.io/foo"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field must be unique for each responder. This field is required.≤
string
Í
priority›int32"«priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.

The responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.

The minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required.≤	
integer˙"
x-kubernetes-map-type	atomic

¡
io.k8s.api.core.v1.ExecActionü"1ExecAction describes a "run in container" action.≤
object ›
⁄
commandŒ"ãCommand is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

¯
!io.k8s.api.core.v1.FCVolumeSource“"´Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.≤
object ï
Œ
fsType√"µfsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.≤
string
A
lun:int32"%lun is Optional: FC target lun number≤	
integer
è
readOnlyÇ"treadOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
à

targetWWNsz"8targetWWNs is Optional: FC target worldwide names (WWNs)≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

·
wwids◊"îwwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

æ
"io.k8s.api.core.v1.FileKeySelectoró".FileKeySelector selects a key of the env file.ö
volumeNameöpathökey≤
object ô
ú
keyî"ÜThe key within the env file. An invalid key will prevent the pod from starting. The keys defined within a source may consist of any printable ASCII characters except '='. During Alpha stage of the EnvFiles feature gate, the key size is limited to 128 characters.≤
string
ã
optional˛"ÔSpecify whether the file or its key must be defined. If the file or key does not exist, then the env var is not published. If optional is set to true and the specified key does not exist, the environment variable will not be set in the Pod's containers.

If optional is set to false and the specified key does not exist, an error will be returned during Pod creation.≤	
boolean
ó
pathé"ÄThe path within the volume from which to select the file. Must be relative and may not contain the '..' path or start with '..'.≤
string
P

volumeNameB"5The name of the volume mount containing the env file.≤
string˙"
x-kubernetes-map-type	atomic

≤
#io.k8s.api.core.v1.FlexVolumeSourceä"hFlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.ödriver≤
object à
O
driverE"8driver is the name of the driver to use for this volume.≤
string
“
fsType«"πfsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.≤
string
k
options`"Coptions is Optional: this field holds extra command options if any.™
≤
string≤
object
è
readOnlyÇ"treadOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
‡
	secretRef“
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"òsecretRef is Optional: secretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
è
&io.k8s.api.core.v1.FlockerVolumeSource‰"√Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.≤
object è
ò
datasetNameà"{datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated≤
string
r
datasetUUIDc"VdatasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset≤
string
Ë
0io.k8s.api.core.v1.GCEPersistentDiskVolumeSource≥"¡Represents a Persistent Disk resource in Google Compute Engine.

A GCE PD must exist before mounting to a container. The disk must also be in the same GCE project and zone as the kubelet. A GCE PD can only be mounted as read/write once or read-only many times. GCE PDs support ownership management and SELinux relabeling.öpdName≤
object ◊
À
fsType¿"≤fsType is filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk≤
string
Ü
	partition¯int32"‚partition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk≤	
integer
º
pdName±"£pdName is unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk≤
string
æ
readOnly±"¢readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk≤	
boolean
Ë
io.k8s.api.core.v1.GRPCAction∆"8GRPCAction specifies an action involving a GRPC service.öport≤
object ˆ
ö
modeë"Émode specifies the connection mode for the gRPC health probe. Set to "TLS" to use TLS without certificate verification. Set to "Plaintext" to use a plaintext (insecure) connection explicitly. If not specified, the probe uses a plaintext (insecure) connection.≤
string
e
port]int32"HPort number of the gRPC service. Number must be in the range 1 to 65535.≤	
integer
Ô
service„"’Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).

If this is not specified, the default behavior is defined by gRPC.≤
string
‰
&io.k8s.api.core.v1.GitRepoVolumeSourceπ"˜Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.

DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.ö
repository≤
object £
ú
	directoryé"Ädirectory is the target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.≤
string
0

repository""repository is the URL≤
string
P
revisionD"7revision is the commit hash for the specified revision.≤
string
ì
(io.k8s.api.core.v1.GlusterfsVolumeSourceÊ"ãRepresents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.ö	endpointsöpath≤
object ∂
Y
	endpointsL"?endpoints is the endpoint name that details Glusterfs topology.≤
string
É
path{"npath is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod≤
string
“
readOnly≈"∂readOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod≤	
boolean
˚
 io.k8s.api.core.v1.HTTPGetAction÷"=HTTPGetAction describes an action based on HTTP Get requests.öport≤
object Å
}
hostu"hHost name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.≤
string
∑
httpHeadersß"CCustom headers to set in the request. HTTP allows repeated headers.≤
array∫/
-
+#/definitions/io.k8s.api.core.v1.HTTPHeader˙#
x-kubernetes-list-type	atomic

7
path/""Path to access on the HTTP server.≤
string
«
portæ
=#/definitions/io.k8s.apimachinery.pkg.util.intstr.IntOrString"}Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
o
protocolc"VProtocol selects the wire protocol for the probe connection. Nil defaults to HTTP/1.1.≤
string
R
schemeH";Scheme to use for connecting to the host. Defaults to HTTP.≤
string
¬
io.k8s.api.core.v1.HTTPHeader†">HTTPHeader describes a custom header to be used in HTTP probesönameövalue≤
object ¬
ë
nameà"{The header field name. This will be canonicalized upon output, so case-variant names will be understood as the same header.≤
string
,
value#"The header field value≤
string
—
io.k8s.api.core.v1.HostAlias∞"oHostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.öip≤
object ´
r
	hostnamese"#Hostnames for the above IP address.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

5
ip/""IP address of the host file entry.≤
string
≠
io.k8s.api.core.v1.HostIPè"<HostIP represents a single IP address allocated to the host.öip≤
object >
<
ip6")IP is the IP address assigned to the host≤
string
â
'io.k8s.api.core.v1.HostPathVolumeSource›"vRepresents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.öpath≤
object œ
≈
pathº"Æpath of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath≤
string
Ñ
type|"otype for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath≤
string
ù
$io.k8s.api.core.v1.ISCSIVolumeSourceÙ"íRepresents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.ötargetPortalöiqnölun≤
object µ
p
chapAuthDiscovery["MchapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication≤	
boolean
j
chapAuthSessionW"IchapAuthSession defines whether support iSCSI Session CHAP authentication≤	
boolean
√
fsType∏"™fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi≤
string
È
initiatorName◊"…initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.≤
string
;
iqn4"'iqn is the target iSCSI Qualified Name.≤
string
~
iscsiInterfacel"_iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).≤
string
C
lun<int32"'lun represents iSCSI Target Lun number.≤	
integer
Ë
portals‹"ôportals is the iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

k
readOnly_"QreadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.≤	
boolean
ë
	secretRefÉ
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"JsecretRef is the CHAP Secret for iSCSI target and initiator authentication
¥
targetPortal£"ïtargetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).≤
string
∫	
$io.k8s.api.core.v1.ImageVolumeSourceë	"5ImageVolumeSource represents a image volume resource.≤
object À
∫

pullPolicy´"ùPolicy for pulling OCI objects. Possible values are: Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.≤
string
ã
	reference˝"ÔRequired: Image or artifact reference to be used. Behaves in the same way as pod.spec.containers[*].image. Pull secrets will be assembled in the same way as for the container image by looking up node credentials, SA image pull secrets, and pod spec image pull secrets. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.≤
string
Â
$io.k8s.api.core.v1.ImageVolumeStatusº";ImageVolumeStatus represents the image-based volume status.öimageRef≤
object Â
‚
imageRef’"«ImageRef is the digest of the image used for this volume. It should have a value that's similar to the pod's status.containerStatuses[i].imageID. The ImageRef length should not exceed 256 characters.≤
string
î
io.k8s.api.core.v1.KeyToPathÛ",Maps a string key to a path within a volume.ökeyöpath≤
object ©
.
key'"key is the key to project.≤
string
√
mode∫int32"§mode is Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.≤	
integer
∑
pathÆ"†path is the relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.≤
string
ˆ
userÌint64"◊user is Optional: The owner UID of the created file. If specified, the item-level user field takes precedence over defaultUser. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
Ú
io.k8s.api.core.v1.Lifecycle—"™Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.≤
object ï
ä
	postStart¸
1#/definitions/io.k8s.api.core.v1.LifecycleHandler"∆PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
É
preStop˜
1#/definitions/io.k8s.api.core.v1.LifecycleHandler"¡PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
ˇ

stopSignal"‚StopSignal defines which signal will be sent to a container when it is being stopped. If not specified, the default is defined by the container runtime in use. StopSignal can only be set for Pods with a non-empty .spec.os.name≤
string
≤
#io.k8s.api.core.v1.LifecycleHandlerä"òLifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.≤
object ‡
l
execd
+#/definitions/io.k8s.api.core.v1.ExecAction"5Exec specifies a command to execute in the container.
n
httpGetc
.#/definitions/io.k8s.api.core.v1.HTTPGetAction"1HTTPGet specifies an HTTP GET request to perform.
u
sleepl
,#/definitions/io.k8s.api.core.v1.SleepAction"<Sleep represents a duration that the container should sleep.
à
	tcpSocket˙
0#/definitions/io.k8s.api.core.v1.TCPSocketAction"≈Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for backward compatibility. There is no validation of this field and lifecycle hooks will fail at runtime when it is specified.
∑
%io.k8s.api.core.v1.LinuxContainerUserç"KLinuxContainerUser represents user identity information in Linux containersöuidögid≤
object •
k
giddint64"OGID is the primary gid initially attached to the first process in the container≤	
integer
»
supplementalGroups±"gSupplementalGroups are the supplemental groups initially attached to the first process in the container≤
array∫
int64≤	
integer˙#
x-kubernetes-list-type	atomic

k
uiddint64"OUID is the primary uid initially attached to the first process in the container≤	
integer
°	
&io.k8s.api.core.v1.LoadBalancerIngressˆ"êLoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.≤
object ‘
{
hostnameo"bHostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)≤
string
{
ipu"hIP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)≤
string
ı
ipModeÍ"‹IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified. Setting this to "VIP" indicates that traffic is delivered to the node with the destination set to the load-balancer's IP and port. Setting this to "Proxy" indicates that traffic is delivered to the node or pod with the destination set to the node's IP and node port or the pod's IP and port. Service implementations may use this information to adjust traffic routing.≤
string
ﬂ
ports’"qPorts is a list of records of service ports If used, every port defined in the service should have an entry in it≤
array∫/
-
+#/definitions/io.k8s.api.core.v1.PortStatus˙#
x-kubernetes-list-type	atomic

ˇ
%io.k8s.api.core.v1.LoadBalancerStatus’"<LoadBalancerStatus represents the status of a load-balancer.≤
object à
Ö
ingress˘"ãIngress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.≤
array∫8
6
4#/definitions/io.k8s.api.core.v1.LoadBalancerIngress˙#
x-kubernetes-list-type	atomic

â
'io.k8s.api.core.v1.LocalObjectReference›"sLocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.≤
object ¥
±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string˙"
x-kubernetes-map-type	atomic

ß
"io.k8s.api.core.v1.NFSVolumeSourceÄ"ÄRepresents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support ownership management or SELinux relabeling.öserveröpath≤
object ﬁ
Ä
pathx"kpath that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs≤
string
√
readOnly∂"ßreadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs≤	
boolean
í
serverá"zserver is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs≤
string
„
io.k8s.api.core.v1.Namespace¬"MNamespace provides a scope for Names. Use of multiple namespaces is optional.≤
object ê

æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
÷
specÕ
.#/definitions/io.k8s.api.core.v1.NamespaceSpec"öSpec defines the behavior of the Namespace. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
‚
status◊
0#/definitions/io.k8s.api.core.v1.NamespaceStatus"¢Status describes the current status of a Namespace. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status˙Q
x-kubernetes-group-version-kind.,- group: ""
  kind: Namespace
  version: v1

‚
%io.k8s.api.core.v1.NamespaceCondition∏"=NamespaceCondition contains details about state of namespace.ötypeöstatus≤
object ⁄
ë
lastTransitionTime{
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"@Last time the condition transitioned from one status to another.
X
messageM"@Human-readable message indicating details about last transition.≤
string
^
reasonT"GUnique, one-word, CamelCase reason for the condition's last transition.≤
string
L
statusB"5Status of the condition, one of True, False, Unknown.≤
string
<
type4"'Type of namespace controller condition.≤
string
‚	
 io.k8s.api.core.v1.NamespaceListΩ	"&NamespaceList is a list of Namespaces.öitems≤
object ¶
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
‘
items "åItems is the list of Namespace objects in the list. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.Namespace
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙U
x-kubernetes-group-version-kind20- group: ""
  kind: NamespaceList
  version: v1


 io.k8s.api.core.v1.NamespaceSpecÀ"6NamespaceSpec describes the attributes on a Namespace.≤
object Ñ
Å

finalizersÚ"ØFinalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

‡
"io.k8s.api.core.v1.NamespaceStatusπ"GNamespaceStatus is information about the current status of a Namespace.≤
object ·
¬

conditions≥"LRepresents the latest available observations of a namespace's current state.≤
array∫7
5
3#/definitions/io.k8s.api.core.v1.NamespaceCondition˙'
x-kubernetes-list-map-keys	- type
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keytype
˙'
x-kubernetes-patch-strategymerge

ô
phaseè"ÅPhase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/≤
string
Ù
io.k8s.api.core.v1.Nodeÿ"iNode is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in etcd).≤
object è

æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
ø
spec∂
)#/definitions/io.k8s.api.core.v1.NodeSpec"àSpec defines the behavior of a node. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
¯
statusÌ
+#/definitions/io.k8s.api.core.v1.NodeStatus"ΩMost recently observed status of the node. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status˙L
x-kubernetes-group-version-kind)'- group: ""
  kind: Node
  version: v1

˚
io.k8s.api.core.v1.NodeAddressÿ"8NodeAddress contains information for the node's address.ötypeöaddress≤
object 
)
address"The node address.≤
string
R
typeJ"=Node address type, one of Hostname, ExternalIP or InternalIP.≤
string
™

io.k8s.api.core.v1.NodeAffinityÜ
";Node affinity is a group of node affinity scheduling rules.≤
object ∫	
Ñ
/preferredDuringSchedulingIgnoredDuringExecution–"ﬁThe scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.≤
array∫<
:
8#/definitions/io.k8s.api.core.v1.PreferredSchedulingTerm˙#
x-kubernetes-list-type	atomic

∞
.requiredDuringSchedulingIgnoredDuringExecution˝
-#/definitions/io.k8s.api.core.v1.NodeSelector"ÀIf the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
ï
1io.k8s.api.core.v1.NodeAllocatableMappedResourcesﬂ"VNodeAllocatableMappedResources describes mapped node allocatable resource allocations.önameöquantity≤
object Ê
J
nameB"5Name is the name of the resource (e.g., cpu, memory).≤
string
ó
quantityä
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity" Quantity is the total node allocatable resource capacity allocated for the claim. This claim's allocated devices is shared by all the containers referencing the claim. Kubelet adds this value to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.
µ
3io.k8s.api.core.v1.NodeAllocatableOverheadResources˝"SNodeAllocatableOverheadResources describes auxiliary overhead resource allocations.öname≤
object í
J
nameB"5Name is the name of the resource (e.g., cpu, memory).≤
string
¸
perContainerÎ
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity"´PerContainer is the variable overhead quantity applied for each container referencing the claim. The container references are recorded in `nodeAllocatableResourceClaimStatuses.containers`. The total overhead quantity allocated for the claim is computed as: Quantity = PerPod + (PerContainer * NumReferences) Kubelet accounts for this overhead in cgroups: - Pod-level cgroup (requests and limits): Kubelet adds PerPod + (PerContainer * NumReferences). - Container-level cgroup (limits only): Kubelet adds PerPod + PerContainer for each referencing container. This allows any single container to access the pod-level overhead, while the parent cgroup caps the total usage to account for PerPod exactly once. At least one of PerPod or PerContainer must be specified. Specifying neither is an invalid configuration.
ƒ
perPodπ
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity"˘PerPod is the flat overhead quantity allocated per pod. Adding to each container limit allows individual containers to utilize the overhead, while the parent pod-level cgroup limit caps the total usage at the pod boundary where the overhead is accounted for exactly once. At least one of PerPod or PerContainer must be specified. Specifying neither is an invalid configuration.
û
5io.k8s.api.core.v1.NodeAllocatableResourceClaimStatus‰
"hNodeAllocatableResourceClaimStatus describes the status of node allocatable resources allocated via DRA.öresourceClaimName≤
object ◊	
†

containersë"RContainers lists the names of all containers in this pod that reference the claim.≤
array∫
≤
string˙ 
x-kubernetes-list-typeset

≈
mappingπ"≈Mapping contains allocations through devices mapped in the device spec's `nodeAllocatableResources[...].mapping` field. This is used by kubelet for pod level and container-level cgroup enforcement.≤
array∫C
A
?#/definitions/io.k8s.api.core.v1.NodeAllocatableMappedResources˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

 
overheadΩ"«Overhead contains allocations through devices mapped in the device spec's `nodeAllocatableResources[...].overhead` field. This is used by kubelet for pod level and container-level cgroup enforcement.≤
array∫E
C
A#/definitions/io.k8s.api.core.v1.NodeAllocatableOverheadResources˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

ú
resourceClaimNameÜ"yResourceClaimName is the resource claim referenced by the pod that resulted in this node allocatable resource allocation.≤
string
≤
 io.k8s.api.core.v1.NodeConditionç"8NodeCondition contains condition information for a node.ötypeöstatus≤
object ¥
Ä
lastHeartbeatTimek
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"0Last time we got an update on a given condition.
å
lastTransitionTimev
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time";Last time the condition transit from one status to another.
X
messageM"@Human readable message indicating details about last transition.≤
string
J
reason@"3(brief) reason for the condition's last transition.≤
string
L
statusB"5Status of the condition, one of True, False, Unknown.≤
string
,
type$"Type of node condition.≤
string
À
#io.k8s.api.core.v1.NodeConfigSource£"óNodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22≤
object {
y
	configMapl
:#/definitions/io.k8s.api.core.v1.ConfigMapNodeConfigSource".ConfigMap is a reference to a Node's ConfigMap
Ï
#io.k8s.api.core.v1.NodeConfigStatusƒ"WNodeConfigStatus describes the status of the config assigned by Node.Spec.ConfigSource.≤
object ‹
¡
active∂
1#/definitions/io.k8s.api.core.v1.NodeConfigSource"ÄActive reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current LastKnownGood config, depending on whether attempting to use the Assigned config results in an error.
˘
assignedÏ
1#/definitions/io.k8s.api.core.v1.NodeConfigSource"∂Assigned reports the checkpointed config the node will try to use. When Node.Spec.ConfigSource is updated, the node checkpoints the associated config payload to local disk, along with a record indicating intended config. The node refers to this record to choose its config checkpoint, and reports this record in Assigned. Assigned only updates in the status after the record has been checkpointed to disk. When the Kubelet is restarted, it tries to make the Assigned config the Active config by loading and validating the checkpointed payload identified by Assigned.
†
errorñ"àError describes any problems reconciling the Spec.ConfigSource to the Active config. Errors may occur, for example, attempting to checkpoint Spec.ConfigSource to the local Assigned record, attempting to checkpoint the payload associated with Spec.ConfigSource, attempting to load or validate the Assigned config, etc. Errors may occur at different points while syncing config. Earlier errors (e.g. download or checkpointing errors) will not result in a rollback to LastKnownGood, and may resolve across Kubelet retries. Later errors (e.g. loading or validating a checkpointed config) will result in a rollback to LastKnownGood. In the latter case, it is usually possible to resolve the error by fixing the config assigned in Spec.ConfigSource. You can find additional information for debugging by searching the error message in the Kubelet log. Error is a human-readable description of the error state; machines can check whether or not Error is empty, but should not rely on the stability of the Error text across Kubelet versions.≤
string
ˆ
lastKnownGood‰
1#/definitions/io.k8s.api.core.v1.NodeConfigSource"ÆLastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned config becomes the LastKnownGood config when the node determines that the Assigned config is stable and correct. This is currently implemented as a 10-minute soak period starting when the local record of Assigned config is updated. If the Assigned config is Active at the end of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil, because the local default config is always assumed good. You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.

&io.k8s.api.core.v1.NodeDaemonEndpoints≈"FNodeDaemonEndpoints lists ports opened by daemons running on the Node.≤
object o
m
kubeletEndpointZ
/#/definitions/io.k8s.api.core.v1.DaemonEndpoint"'Endpoint on which Kubelet is listening.
ò
io.k8s.api.core.v1.NodeFeaturesÙ" NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.≤
object ò
ï
supplementalGroupsPolicyy"kSupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.≤	
boolean
˙
io.k8s.api.core.v1.NodeList⁄"ONodeList is the whole list of all Nodes which have been registered with master.öitems≤
object ü
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
N
itemsE"List of nodes≤
array∫)
'
%#/definitions/io.k8s.api.core.v1.Node
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙P
x-kubernetes-group-version-kind-+- group: ""
  kind: NodeList
  version: v1

˙
*io.k8s.api.core.v1.NodePodPreemptionPolicyÀ"cNodePodPreemptionPolicy defines the node-level policies governing preemption for pods on this node.≤
object ◊
‘
disableResizePreemption∏"¯DisableResizePreemption lists the owners (e.g., autoscalers, operators, administrators) that have requested to disable scheduler and Kubelet preemption for in-place pod resize on this node. If this list is non-empty, resize-induced preemption is disabled on this node. This is an alpha field and requires enabling the InPlacePodVerticalScalingSchedulerPreemption feature gate.≤
array∫
≤
string˙ 
x-kubernetes-list-typeset

©
%io.k8s.api.core.v1.NodeRuntimeHandlerˇ";NodeRuntimeHandler is a set of runtime handler information.≤
object ≥
^
featuresR
;#/definitions/io.k8s.api.core.v1.NodeRuntimeHandlerFeatures"Supported features.
Q
nameI"<Runtime handler name. Empty for the default runtime handler.≤
string
®
-io.k8s.api.core.v1.NodeRuntimeHandlerFeaturesˆ"SNodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.≤
object í
à
recursiveReadOnlyMountsm"_RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.≤	
boolean
Ñ
userNamespacesr"dUserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.≤	
boolean
Í
io.k8s.api.core.v1.NodeSelector∆"ºA node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.önodeSelectorTerms≤
object ø
º
nodeSelectorTerms¶"<Required. A list of node selector terms. The terms are ORed.≤
array∫5
3
1#/definitions/io.k8s.api.core.v1.NodeSelectorTerm˙#
x-kubernetes-list-type	atomic
˙"
x-kubernetes-map-type	atomic

æ
*io.k8s.api.core.v1.NodeSelectorRequirementè"wA node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.ökeyöoperator≤
object ˆ
?
key8"+The label key that the selector applies to.≤
string
é
operatorÅ"tRepresents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.≤
string
°
valuesñ"”An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

˝
#io.k8s.api.core.v1.NodeSelectorTerm’"•A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.≤
object ˘
º
matchExpressionsß"6A list of node selector requirements by node's labels.≤
array∫<
:
8#/definitions/io.k8s.api.core.v1.NodeSelectorRequirement˙#
x-kubernetes-list-type	atomic

∑
matchFieldsß"6A list of node selector requirements by node's fields.≤
array∫<
:
8#/definitions/io.k8s.api.core.v1.NodeSelectorRequirement˙#
x-kubernetes-list-type	atomic
˙"
x-kubernetes-map-type	atomic


io.k8s.api.core.v1.NodeSpec–">NodeSpec describes the attributes that a node is created with.≤
object Å
”
configSource¬
1#/definitions/io.k8s.api.core.v1.NodeConfigSource"åDeprecated: Previously used to specify the source of the node's configuration for the DynamicKubeletConfig feature. This feature is removed.
Ü

externalIDx"kDeprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966≤
string
Q
podCIDRF"9PodCIDR represents the pod IP range assigned to the node.≤
string
œ
podCIDRs¬"ÿpodCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.≤
array∫
≤
string˙ 
x-kubernetes-list-typeset
˙'
x-kubernetes-patch-strategymerge

ò
podPreemptionPolicyÄ
8#/definitions/io.k8s.api.core.v1.NodePodPreemptionPolicy"√PodPreemptionPolicy controls the node-level preemption behaviors for pods on this node. This is an alpha field and requires enabling the InPlacePodVerticalScalingSchedulerPreemption feature gate.
Å

providerIDs"fID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>≤
string
â
taints" If specified, the node's taints.≤
array∫*
(
&#/definitions/io.k8s.api.core.v1.Taint˙#
x-kubernetes-list-type	atomic

œ
unschedulableΩ"ÆUnschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration≤	
boolean
„
io.k8s.api.core.v1.NodeStatus¡"=NodeStatus is information about the current status of a node.≤
object Û
˛
	addresses"èList of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/reference/node/node-status/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).≤
array∫0
.
,#/definitions/io.k8s.api.core.v1.NodeAddress˙'
x-kubernetes-list-map-keys	- type
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keytype
˙'
x-kubernetes-patch-strategymerge

∆
allocatable∂"gAllocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
Ÿ
capacityÃ"}Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
˘

conditionsÍ"áConditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/reference/node/node-status/#condition≤
array∫2
0
.#/definitions/io.k8s.api.core.v1.NodeCondition˙'
x-kubernetes-list-map-keys	- type
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keytype
˙'
x-kubernetes-patch-strategymerge

ë
configÜ
1#/definitions/io.k8s.api.core.v1.NodeConfigStatus"QStatus of the config assigned to the node via the dynamic Kubelet config feature.
t
daemonEndpointsa
4#/definitions/io.k8s.api.core.v1.NodeDaemonEndpoints")Endpoints of daemons running on the Node.
∑
declaredFeatures¢"`DeclaredFeatures represents the features related to feature gates that are declared by the node.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ä
features~
-#/definitions/io.k8s.api.core.v1.NodeFeatures"MFeatures describes the set of features implemented by the CRI implementation.
ò
imagesç"%List of container images on this node≤
array∫3
1
/#/definitions/io.k8s.api.core.v1.ContainerImage˙#
x-kubernetes-list-type	atomic

∂
nodeInfo©
/#/definitions/io.k8s.api.core.v1.NodeSystemInfo"vSet of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/reference/node/node-status/#info
Õ
phase√"µNodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.≤
string
ü
runtimeHandlersã"The available runtime handlers.≤
array∫7
5
3#/definitions/io.k8s.api.core.v1.NodeRuntimeHandler˙#
x-kubernetes-list-type	atomic

™
volumesAttachedñ".List of volumes that are attached to the node.≤
array∫3
1
/#/definitions/io.k8s.api.core.v1.AttachedVolume˙#
x-kubernetes-list-type	atomic

ä
volumesInUsez"8List of attachable volumes in use (mounted) by the node.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

∞
!io.k8s.api.core.v1.NodeSwapStatusä"2NodeSwapStatus represents swap memory information.≤
object H
F
capacity:int64"%Total amount of swap memory in bytes.≤	
integer
˝
!io.k8s.api.core.v1.NodeSystemInfo◊"CNodeSystemInfo is a set of ids/uuids to uniquely identify the node.ö	machineIDö
systemUUIDöbootIDökernelVersionöosImageöcontainerRuntimeVersionökubeletVersionökubeProxyVersionöoperatingSystemöarchitecture≤
object Ë

B
architecture2"%The Architecture reported by the node≤
string
4
bootID*"Boot ID reported by the node.≤
string
ã
containerRuntimeVersionp"cContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).≤
string
n
kernelVersion]"PKernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).≤
string
T
kubeProxyVersion@"3Deprecated: KubeProxy Version reported by the node.≤
string
D
kubeletVersion2"%Kubelet Version reported by the node.≤
string
ﬂ
	machineID—"√MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html≤
string
I
operatingSystem6")The Operating System reported by the node≤
string
n
osImagec"VOS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).≤
string
X
runningInUserNamespace>"0Whether the node is running in a user namespace.≤	
boolean
Z
swapR
/#/definitions/io.k8s.api.core.v1.NodeSwapStatus"Swap Info reported by the node.
˛

systemUUIDÔ"·SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid≤
string
Î
&io.k8s.api.core.v1.ObjectFieldSelector¿"?ObjectFieldSelector selects an APIVersioned field of an object.ö	fieldPath≤
object ø
h

apiVersionZ"MVersion of the schema the FieldPath is written in terms of, defaults to "v1".≤
string
S
	fieldPathF"9Path of the field to select in the specified API version.≤
string˙"
x-kubernetes-map-type	atomic

È
"io.k8s.api.core.v1.ObjectReference¬"]ObjectReference contains enough information to let you inspect or modify the referred object.≤
object Ø
7

apiVersion)"API version of the referent.≤
string
‘
	fieldPath∆"∏If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.≤
string
ó
kindé"ÄKind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
Ñ
name|"oName of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
é
	namespaceÄ"sNamespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/≤
string
Ê
resourceVersion“"ƒSpecific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency≤
string
Å
uidz"mUID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids≤
string˙"
x-kubernetes-map-type	atomic

†#
,io.k8s.api.core.v1.PersistentVolumeClaimSpecÔ""ÅPersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes≤
object ‹!
Ò
accessModes·"ûaccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

à

dataSource˘
:#/definitions/io.k8s.api.core.v1.TypedLocalObjectReference"∫dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.
Ô
dataSourceRef›
5#/definitions/io.k8s.api.core.v1.TypedObjectReference"£dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef
  allows any non-core object, as well as PersistentVolumeClaim objects.
* While dataSource ignores disallowed values (dropping them), dataSourceRef
  preserves all values, and generates an error if a disallowed value is
  specified.
* While dataSource only allows local objects, dataSourceRef allows objects
  in any namespaces.
(Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
å
	resources˛
;#/definitions/io.k8s.api.core.v1.VolumeResourceRequirements"æresources represents the minimum resources the volume should have. Users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
ê
selectorÉ
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector"?selector is a label query over volumes to consider for binding.
º
storageClassNameß"ôstorageClassName is the name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1≤
string
˚
volumeAttributesClassName›"œvolumeAttributesClassName may be used to set the VolumeAttributesClass used by this claim. If specified, the CSI driver will create or update the volume with the attributes defined in the corresponding VolumeAttributesClass. This has a different purpose than storageClassName, it can be changed after the claim is created. An empty string or nil value indicates that no VolumeAttributesClass will be applied to the claim. If the claim enters an Infeasible error state, this field can be reset to its previous value (including nil) to cancel the modification. If the resource referred to by volumeAttributesClass does not exist, this PersistentVolumeClaim will be set to a Pending state, as reflected by the modifyVolumeStatus field, until such as a resource exists. More info: https://kubernetes.io/docs/concepts/storage/volume-attributes-classes/≤
string
ù

volumeModeé"ÄvolumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.≤
string
j

volumeName\"OvolumeName is the binding reference to the PersistentVolume backing this claim.≤
string
√
0io.k8s.api.core.v1.PersistentVolumeClaimTemplateé"sPersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects as part of an EphemeralVolumeSource.öspec≤
object É
Ë
metadata€
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ôMay contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
ï
specå
:#/definitions/io.k8s.api.core.v1.PersistentVolumeClaimSpec"ÕThe specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
Ω
4io.k8s.api.core.v1.PersistentVolumeClaimVolumeSourceÑ"ûPersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system).ö	claimName≤
object »
·
	claimName”"≈claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims≤
string
b
readOnlyV"HreadOnly Will force the ReadOnly setting in VolumeMounts. Default false.≤	
boolean
∞
3io.k8s.api.core.v1.PhotonPersistentDiskVolumeSource¯"8Represents a Photon Controller persistent disk resource.öpdID≤
object ®
Œ
fsType√"µfsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.≤
string
U
pdIDM"@pdID is the ID that identifies Photon Controller persistent disk≤
string
∂
io.k8s.api.core.v1.Podõ"wPod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.≤
object ≈

æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
÷
specÕ
(#/definitions/io.k8s.api.core.v1.PodSpec"†Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
ó
statuså
*#/definitions/io.k8s.api.core.v1.PodStatus"›Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status˙K
x-kubernetes-group-version-kind(&- group: ""
  kind: Pod
  version: v1

ä
io.k8s.api.core.v1.PodAffinityÁ"?Pod affinity is a group of inter pod affinity scheduling rules.≤
object ó
í
/preferredDuringSchedulingIgnoredDuringExecutionﬁ"ÏThe scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.≤
array∫<
:
8#/definitions/io.k8s.api.core.v1.WeightedPodAffinityTerm˙#
x-kubernetes-list-type	atomic

ˇ
.requiredDuringSchedulingIgnoredDuringExecutionÃ"‚If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.PodAffinityTerm˙#
x-kubernetes-list-type	atomic

ÿ
"io.k8s.api.core.v1.PodAffinityTerm±"ﬂDefines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is runningötopologyKey≤
object ≤
»
labelSelector∂
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector"rA label query over a set of resources, in this case pods. If it's null, this PodAffinityTerm matches with no Pods.
´
matchLabelKeysò"’MatchLabelKeys is a set of pod label keys to select which pods will be taken into consideration. The keys are used to lookup values from the incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)` to select the group of existing pods which pods will be taken into consideration for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming pod labels will be ignored. The default value is empty. The same key is forbidden to exist in both matchLabelKeys and labelSelector. Also, matchLabelKeys cannot be set when labelSelector isn't set.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

∫
mismatchLabelKeys§"·MismatchLabelKeys is a set of pod label keys to select which pods will be taken into consideration. The keys are used to lookup values from the incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)` to select the group of existing pods which pods will be taken into consideration for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming pod labels will be ignored. The default value is empty. The same key is forbidden to exist in both mismatchLabelKeys and labelSelector. Also, mismatchLabelKeys cannot be set when labelSelector isn't set.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ë
namespaceSelector˚
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector"∂A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces.
Ï

namespaces›"önamespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace".≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ˆ
topologyKeyÊ"ÿThis pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.≤
string
≥
"io.k8s.api.core.v1.PodAntiAffinityå"IPod anti affinity is a group of inter pod anti affinity scheduling rules.≤
object ≤
£
/preferredDuringSchedulingIgnoredDuringExecutionÔ"˝The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and subtracting "weight" from the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.≤
array∫<
:
8#/definitions/io.k8s.api.core.v1.WeightedPodAffinityTerm˙#
x-kubernetes-list-type	atomic

â
.requiredDuringSchedulingIgnoredDuringExecution÷"ÏIf the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.PodAffinityTerm˙#
x-kubernetes-list-type	atomic

Ó
+io.k8s.api.core.v1.PodCertificateProjectionæ"\PodCertificateProjection provides a private key and X.509 certificate in the pod filesystem.ö
signerNameökeyType≤
object ∫
”
certificateChainPath∫"¨Write the certificate chain at this path in the projected volume.

Most applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.≤
string
°
credentialBundlePathà"˙Write the credential bundle at this path in the projected volume.

The credential bundle is a single file that contains multiple PEM blocks. The first PEM block is a PRIVATE KEY block, containing a PKCS#8 private key.

The remaining blocks are CERTIFICATE blocks, containing the issued certificate chain from the signer (leaf and any intermediates).

Using credentialBundlePath lets your Pod's application code make a single atomic read that retrieves a consistent key and certificate chain.  If you project them to separate files, your application code will need to additionally check that the leaf certificate was issued to the key.≤
string
∏
keyPath¨"ûWrite the key at this path in the projected volume.

Most applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.≤
string
Æ
keyType¢"îThe type of keypair Kubelet will generate for the pod.

Valid values are "RSA3072", "RSA4096", "ECDSAP256", "ECDSAP384", "ECDSAP521", and "ED25519".≤
string
∫
maxExpirationSeconds°int32"ãmaxExpirationSeconds is the maximum lifetime permitted for the certificate.

Kubelet copies this value verbatim into the PodCertificateRequests it generates for this projection.

If omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).

The signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.≤	
integer
U

signerNameG":Kubelet's generated CSRs will be addressed to this signer.≤
string
ˆ
userÌint64"◊user is Optional: The owner UID of the created file. If specified, the item-level user field takes precedence over defaultUser. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
ƒ
userAnnotations∞"íuserAnnotations allow pod authors to pass additional information to the signer implementation.  Kubernetes does not restrict or validate this metadata in any way.

These values are copied verbatim into the `spec.unverifiedUserAnnotations` field of the PodCertificateRequest objects that Kubelet creates.

Entries are subject to the same validation as object metadata annotations, with the addition that all keys must be domain-prefixed. No restrictions are placed on values, except an overall size limitation on the entire field.

Signers should document the keys and values they support. Signers should deny requests that contain keys they do not recognize.™
≤
string≤
object
†
io.k8s.api.core.v1.PodCondition¸"DPodCondition contains details for the current condition of this pod.ötypeöstatus≤
object ó
n
lastProbeTime]
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time""Last time we probed the condition.
ë
lastTransitionTime{
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"@Last time the condition transitioned from one status to another.
X
messageM"@Human-readable message indicating details about last transition.≤
string
Ü
observedGenerationpint64"[If set, this represents the .metadata.generation that the pod condition was set based upon.≤	
integer
^
reasonT"GUnique, one-word, CamelCase reason for the condition's last transition.≤
string
∑
status¨"ûStatus is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions≤
string
ì
typeä"}Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions≤
string
÷
io.k8s.api.core.v1.PodDNSConfig≤"_PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.≤
object ¬
Í
nameservers⁄"óA list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

€
optionsœ"‚A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.≤
array∫7
5
3#/definitions/io.k8s.api.core.v1.PodDNSConfigOption˙#
x-kubernetes-list-type	atomic

Ù
searchesÁ"§A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ˇ
%io.k8s.api.core.v1.PodDNSConfigOption’"9PodDNSConfigOption defines DNS resolver options of a pod.≤
object ã
G
name?"2Name is this DNS resolver option's name. Required.≤
string
@
value7"*Value is this DNS resolver option's value.≤
string
Í
1io.k8s.api.core.v1.PodExtendedResourceClaimStatus¥"ŒPodExtendedResourceClaimStatus is stored in the PodStatus for the extended resource requests backed by DRA. It stores the generated name for the corresponding special ResourceClaim created by the scheduler.örequestMappingsöresourceClaimName≤
object Æ
ò
requestMappingsÑ"âRequestMappings identifies the mapping of <container, extended resource backed by DRA> to  device request in the generated ResourceClaim.≤
array∫E
C
A#/definitions/io.k8s.api.core.v1.ContainerExtendedResourceRequest˙#
x-kubernetes-list-type	atomic

ê
resourceClaimName{"nResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod.≤
string
©
io.k8s.api.core.v1.PodIPå":PodIP represents a single IP address allocated to the pod.öip≤
object =
;
ip5"(IP is the IP address assigned to the pod≤
string
£	
io.k8s.api.core.v1.PodListÑ	"PodList is a list of Pods.öitems≤
object ˇ
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
≠
items£"lList of pods. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md≤
array∫(
&
$#/definitions/io.k8s.api.core.v1.Pod
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙O
x-kubernetes-group-version-kind,*- group: ""
  kind: PodList
  version: v1

„
io.k8s.api.core.v1.PodOS∆")PodOS defines the OS parameters of a pod.öname≤
object Ö
Ç
name˘"ÎName is the name of the operating system. The currently supported values are linux and windows. Additional value may be defined in future and can be one of: https://github.com/opencontainers/runtime-spec/blob/master/config.md#platform-specific-configuration Clients should expect to handle additional values and treat unrecognized values in this field as os: null≤
string
ı
#io.k8s.api.core.v1.PodReadinessGateÕ":PodReadinessGate contains the reference to a pod conditionöconditionType≤
object s
q
conditionType`"SConditionType refers to a condition in the pod's condition list with matching type.≤
string
¯
#io.k8s.api.core.v1.PodResourceClaim–"ÌPodResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the pod.

It adds a name to it that uniquely identifies the ResourceClaim inside the Pod. Containers that need access to the ResourceClaim reference it with this name.

When the DRAWorkloadResourceClaims feature gate is enabled and this Pod belongs to a PodGroup, a PodResourceClaim is matched to a PodGroupResourceClaim if all of their fields are equal (Name, ResourceClaimName, and ResourceClaimTemplateName). A matched claim references a single ResourceClaim shared across all Pods in the PodGroup, reserved for the PodGroup in ResourceClaimStatus.ReservedFor rather than for individual Pods.öname≤
object  
k
namec"VName uniquely identifies this resource claim inside the pod. This must be a DNS_LABEL.≤
string
À
resourceClaimNameµ"ßResourceClaimName is the name of a ResourceClaim object in the same namespace as this pod.

Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.≤
string
å

resourceClaimTemplateNameÓ	"‡	ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this pod.

The template will be used to create a new ResourceClaim, which will be bound to this pod. When this pod is deleted, the ResourceClaim will also be deleted. The pod name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in pod.status.resourceClaimStatuses.

When the DRAWorkloadResourceClaims feature gate is enabled and the pod belongs to a PodGroup that defines a PodGroupResourceClaim with the same Name and ResourceClaimTemplateName, this PodResourceClaim resolves to the ResourceClaim generated for the PodGroup. All pods in the group that define an equivalent PodResourceClaim matching the PodGroupResourceClaim's Name and ResourceClaimTemplateName share the same generated ResourceClaim. ResourceClaims generated for a PodGroup are owned by the PodGroup and their lifecycles are tied to the PodGroup instead of any individual pod.

This field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.

Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.≤
string
–
)io.k8s.api.core.v1.PodResourceClaimStatus¢"∑PodResourceClaimStatus is stored in the PodStatus for each PodResourceClaim which references a ResourceClaimTemplate. It stores the generated name for the corresponding ResourceClaim.öname≤
object “
«
nameæ"∞Name uniquely identifies this resource claim inside the pod. This must match the name of an entry in pod.spec.resourceClaims, which implies that the string must be a DNS_LABEL.≤
string
Ö
resourceClaimNameÔ"·ResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod.

When the DRAWorkloadResourceClaims feature is enabled and the corresponding PodResourceClaim matches a PodGroupResourceClaim made by the Pod's PodGroup, then this is the name of the ResourceClaim generated and reserved for the PodGroup.

If this is unset, then generating a ResourceClaim was not necessary. The pod.spec.resourceClaims entry can be ignored in this case.≤
string
Ë
$io.k8s.api.core.v1.PodSchedulingGateø"APodSchedulingGate is associated to a Pod to guard its scheduling.öname≤
object g
e
name]"PName of the scheduling gate. Each scheduling gate must have a unique name field.≤
string
î
%io.k8s.api.core.v1.PodSchedulingGroupÍ"—PodSchedulingGroup identifies the runtime scheduling group instance that a Pod belongs to. The scheduler uses this information to apply workload-aware scheduling semantics. Exactly one field must be specified.≤
object ∞
≠
podGroupNameú"éPodGroupName specifies the name of the standalone PodGroup object that represents the runtime instance of this group. Must be a DNS subdomain.≤
string˙T
x-kubernetes-unions=;- fields-to-discriminateBy:
    podGroupName: PodGroupName

¢3
%io.k8s.api.core.v1.PodSecurityContext¯2"ÙPodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.≤
object Ú0
◊
appArmorProfile√
0#/definitions/io.k8s.api.core.v1.AppArmorProfile"éappArmorProfile is the AppArmor options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
â
fsGroup˝int64"ÁA special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:

1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----

If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.≤	
integer
Ò
fsGroupChangePolicyŸ"ÀfsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.≤
string
‰

runAsGroup’int64"øThe GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.≤	
integer
∫
runAsNonRoot©"öIndicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.≤	
boolean
Å
	runAsUserÛint64"›The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.≤	
integer
Õ

seLinuxChangePolicyµ
"ß
seLinuxChangePolicy defines how the container's SELinux label is applied to all volumes used by the Pod. It has no effect on nodes that do not support SELinux or to volumes does not support SELinux. Valid values are "MountOption" and "Recursive".

"Recursive" means relabeling of all files on all Pod volumes by the container runtime. This may be slow for large volumes, but allows mixing privileged and unprivileged Pods sharing the same volume on the same node.

"MountOption" mounts all eligible Pod volumes with `-o context` mount option. This requires all Pods that share the same volume to use the same SELinux label. It is not possible to share the same volume among privileged and unprivileged Pods. Eligible volumes are in-tree FibreChannel and iSCSI volumes, and all CSI volumes whose CSI driver announces SELinux support by setting spec.seLinuxMount: true in their CSIDriver instance. Other volumes are always re-labelled recursively.

If not specified, "MountOption" is used.

This field affects only Pods that have SELinux label set, either in PodSecurityContext or in SecurityContext of all containers.

All Pods that use the same volume should use the same seLinuxChangePolicy, otherwise some pods can get stuck in ContainerCreating state. Note that this field cannot be set when spec.os.name is windows.≤
string
∆
seLinuxOptions≥
/#/definitions/io.k8s.api.core.v1.SELinuxOptions"ˇThe SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
¿
seccompProfile≠
/#/definitions/io.k8s.api.core.v1.SeccompProfile"zThe seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
¢
supplementalGroupsã"¿A list of groups applied to the first process run in each container, in addition to the container's primary GID and fsGroup (if specified).  If the SupplementalGroupsPolicy feature is enabled, the supplementalGroupsPolicy field determines whether these are in addition to or instead of any group memberships defined in the container image. If unspecified, no additional groups are added, though group memberships defined in the container image may still be used, depending on the supplementalGroupsPolicy field. Note that this field cannot be set when spec.os.name is windows.≤
array∫
int64≤	
integer˙#
x-kubernetes-list-type	atomic

¢
supplementalGroupsPolicyÖ"˜Defines how supplemental groups of the first container processes are calculated. Valid values are "Merge" and "Strict". If not specified, "Merge" is used. (Alpha) Using the field requires the SupplementalGroupsPolicy feature gate to be enabled and the container runtime must implement support for this feature. Note that this field cannot be set when spec.os.name is windows.≤
string
∏
sysctls¨"ÀSysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.≤
array∫+
)
'#/definitions/io.k8s.api.core.v1.Sysctl˙#
x-kubernetes-list-type	atomic

å
windowsOptions˘
>#/definitions/io.k8s.api.core.v1.WindowsSecurityContextOptions"∂The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
Êí
io.k8s.api.core.v1.PodSpec∆í""PodSpec is a description of a pod.ö
containers≤
object Öí
¸
activeDeadlineSeconds‚int64"ÃOptional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.≤	
integer
g
affinity[
)#/definitions/io.k8s.api.core.v1.Affinity".If specified, the pod's scheduling constraints
ï
automountServiceAccountTokenu"gAutomountServiceAccountToken indicates whether a service account token should be automatically mounted.≤	
boolean
â

containers˙"õList of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.Container˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

∆
	dnsConfig∏
-#/definitions/io.k8s.api.core.v1.PodDNSConfig"ÜSpecifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
ˆ
	dnsPolicyË"⁄Set DNS policy for the pod. Defaults to "ClusterFirst". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.≤
string
€
enableServiceLinksƒ"µEnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.≤	
boolean
Ê
ephemeralContainersŒ"ÊList of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.≤
array∫7
5
3#/definitions/io.k8s.api.core.v1.EphemeralContainer˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

ë

evictionResponders˙	"ìevictionResponders reference responders that react to Evictions based on EvictionRequests. Responders should observe and communicate through the Eviction Resource API to help with the graceful termination of a pod. The responders are selected sequentially, according to their specified priority.

Responders should periodically report on an eviction progress by updating the .status.responders[].heartbeatTime field of the Eviction object. If this field is not updated within the heartbeat deadline defined by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder with a lower priority. If there is no other responder, the last default imperative-eviction.k8s.io/evictor responder with a priority of 100 will evict the pod using the imperative Eviction API (pods/<name>/eviction subresource).

The maximum length of the responders list is 10. Responders are not supported when the pod is part of a PodGroup (.spec.schedulingGroup is set). This field can only be set on creation and is immutable afterwards.≤
array∫6
4
2#/definitions/io.k8s.api.core.v1.EvictionResponder˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

ÿ
hostAliases»"nHostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.HostAlias˙%
x-kubernetes-list-map-keys- ip
˙ 
x-kubernetes-list-typemap
˙%
x-kubernetes-patch-merge-keyip
˙'
x-kubernetes-patch-strategymerge

R
hostIPCG"9Use the host's ipc namespace. Optional: Default to false.≤	
boolean
ã
hostNetwork˚"ÏHost networking requested for this pod. Use the host's network namespace. When using HostNetwork you should specify ports so the scheduler is aware. When `hostNetwork` is true, specified `hostPort` fields in port definitions must match `containerPort`, and unspecified `hostPort` fields in port definitions are defaulted to match `containerPort`. Default to false.≤	
boolean
R
hostPIDG"9Use the host's pid namespace. Optional: Default to false.≤	
boolean
ò
	hostUsersä"˚Use the host's user namespace. Optional: Default to true. If set to true or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to false, a new userns is created for the pod. Setting false is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host.≤	
boolean
Ü
hostnamez"mSpecifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.≤
string
∏
hostnameOverride£"ïHostnameOverride specifies an explicit override for the pod's hostname as perceived by the pod. This field only specifies the pod's hostname and does not affect its DNS records. When this field is set to a non-empty string: - It takes precedence over the values set in `hostname` and `subdomain`. - The Pod's hostname will be set to this value. - `setHostnameAsFQDN` must be nil or set to false. - `hostNetwork` must be set to false.

This field must be a valid DNS subdomain as defined in RFC 1123 and contain at most 64 characters.≤
string
‘
imagePullSecretsø"’ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod≤
array∫9
7
5#/definitions/io.k8s.api.core.v1.LocalObjectReference˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

”
initContainers¿"·List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.Container˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

«
nodeName∫"¨NodeName indicates in which node this pod is scheduled. If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName. Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod. This field should not be used to express a desire for the pod to be scheduled on a specific node. https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename≤
string
¡
nodeSelector∞"ÌNodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/™
≤
string≤
object˙"
x-kubernetes-map-type	atomic

§

osù

&#/definitions/io.k8s.api.core.v1.PodOS"Ú	Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set.

If the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions

If the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.hostUsers - spec.resources - spec.securityContext.appArmorProfile - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.securityContext.supplementalGroupsPolicy - spec.containers[*].securityContext.appArmorProfile - spec.containers[*].securityContext.seLinuxOptions - spec.containers[*].securityContext.seccompProfile - spec.containers[*].securityContext.capabilities - spec.containers[*].securityContext.readOnlyRootFilesystem - spec.containers[*].securityContext.privileged - spec.containers[*].securityContext.allowPrivilegeEscalation - spec.containers[*].securityContext.procMount - spec.containers[*].securityContext.runAsUser - spec.containers[*].securityContext.runAsGroup
¸
overheadÔ"üOverhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
Ÿ
preemptionPolicyƒ"∂PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. Defaults to PreemptLowerPriority if unset.≤
string
œ
priority¬int32"¨The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.≤	
integer
î
priorityClassName˛"If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.≤
string
ô
readinessGatesÜ"õIf specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates≤
array∫5
3
1#/definitions/io.k8s.api.core.v1.PodReadinessGate˙#
x-kubernetes-list-type	atomic

¬
resourceClaimsØ"æResourceClaims defines which ResourceClaims must be allocated and reserved before the Pod is allowed to start. The resources will be made available to those containers which consume them by name.

This is a stable field but requires that the DynamicResourceAllocation feature gate is enabled.

This field is immutable.≤
array∫5
3
1#/definitions/io.k8s.api.core.v1.PodResourceClaim˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙2
x-kubernetes-patch-strategymerge,retainKeys

å
	resources˛
5#/definitions/io.k8s.api.core.v1.ResourceRequirements"ƒResources is the total amount of CPU and Memory resources required by all containers in the pod. It supports specifying Requests and Limits for "cpu", "memory" and "hugepages-" resource names only. ResourceClaims are not supported.

This field enables fine-grained control over resource allocation for the entire pod, allowing resource sharing among containers in a pod.

This is an alpha field and requires enabling the PodLevelResources feature gate.
£
restartPolicyë"ÉRestart policy for all containers within the pod. One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy≤
string
√
runtimeClassNameÆ"†RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class≤
string
£
schedulerNameë"ÉIf specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.≤
string
∂
schedulingGates¢"ªSchedulingGates is an opaque list of values that if specified will block scheduling the pod. If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the scheduler will not attempt to schedule the pod.

SchedulingGates can only be set at pod creation time, and be removed only afterwards.≤
array∫6
4
2#/definitions/io.k8s.api.core.v1.PodSchedulingGate˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge

»
schedulingGroup¥
3#/definitions/io.k8s.api.core.v1.PodSchedulingGroup"¸SchedulingGroup provides a reference to the immediate scheduling runtime grouping object that this Pod belongs to. This field is used by the scheduler to identify the group and apply the correct group scheduling policies. The association with a group also impacts other lifecycle aspects of a Pod that are relevant in a wider context of scheduling like preemption, resource attachment, etc. If not specified, the Pod is treated as a single unit in all of these aspects. The group object referenced by this field may not exist at the time the Pod is created. This field is immutable, but a group object with the same name may be recreated with different policies. Doing this during pod scheduling may result in the placement not conforming to the expected policies.
Û
securityContextﬂ
3#/definitions/io.k8s.api.core.v1.PodSecurityContext"ßSecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
ë
serviceAccount"rDeprecatedServiceAccount is a deprecated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.≤
string
–
serviceAccountNameπ"´ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/≤
string
Ä
setHostnameAsFQDNÍ"€If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.≤	
boolean
Ù
shareProcessNamespace⁄"ÀShare a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.≤	
boolean
Œ
	subdomain¿"≤If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>". If not specified, the pod will not have a domainname at all.≤
string
˝
terminationGracePeriodSeconds€int64"≈Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.≤	
integer
ò
tolerationsà"$If specified, the pod's tolerations.≤
array∫/
-
+#/definitions/io.k8s.api.core.v1.Toleration˙#
x-kubernetes-list-type	atomic

˛
topologySpreadConstraints‡"–TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.≤
array∫=
;
9#/definitions/io.k8s.api.core.v1.TopologySpreadConstraint˙B
x-kubernetes-list-map-keys$"- topologyKey
- whenUnsatisfiable
˙ 
x-kubernetes-list-typemap
˙.
x-kubernetes-patch-merge-keytopologyKey
˙'
x-kubernetes-patch-strategymerge

˘
volumesÌ"ÜList of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes≤
array∫+
)
'#/definitions/io.k8s.api.core.v1.Volume˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙2
x-kubernetes-patch-strategymerge,retainKeys

ØI
io.k8s.api.core.v1.PodStatuséI"∂PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.≤
object ∆G
ü
allocatedResourcesà"∏AllocatedResources is the total requests allocated for this pod by the node. If pod-level requests are not set, this will be the total requests aggregated across containers in the pod.™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
Ë

conditionsŸ"xCurrent service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions≤
array∫1
/
-#/definitions/io.k8s.api.core.v1.PodCondition˙'
x-kubernetes-list-map-keys	- type
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keytype
˙'
x-kubernetes-patch-strategymerge

Á
containerStatuses—"ÁStatuses of containers in this pod. Each container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.ContainerStatus˙#
x-kubernetes-list-type	atomic

ó
ephemeralContainerStatuses¯"éStatuses for any ephemeral containers that have run in this pod. Each ephemeral container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.ContainerStatus˙#
x-kubernetes-list-type	atomic

í
extendedResourceClaimStatuss
?#/definitions/io.k8s.api.core.v1.PodExtendedResourceClaimStatus"0Status of extended resource claim backed by DRA.
°
hostIPñ"àhostIP holds the IP address of the host to which the pod is assigned. Empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns mean that HostIP will not be updated even if there is a node is assigned to pod≤
string
ï
hostIPsâ"÷hostIPs holds the IP addresses allocated to the host. If this field is specified, the first entry must match the hostIP field. This list is empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns means that HostIPs will not be updated even if there is a node is assigned to this pod.≤
array∫+
)
'#/definitions/io.k8s.api.core.v1.HostIP˙#
x-kubernetes-list-type	atomic
˙%
x-kubernetes-patch-merge-keyip
˙'
x-kubernetes-patch-strategymerge

Ö
initContainerStatusesÎ"ÅStatuses of init containers in this pod. The most recent successful non-restartable init container will have ready = true, the most recently started container will have startTime set. Each init container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-and-container-status≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.ContainerStatus˙#
x-kubernetes-list-type	atomic

k
message`"SA human readable message indicating details about why the pod is in this condition.≤
string
‡
$nodeAllocatableResourceClaimStatuses∑"•NodeAllocatableResourceClaimStatuses contains the status of node-allocatable resources that were allocated for this pod through DRA claims. This includes resources currently reported in v1.Node `status.allocatable` that are not extended resources (see https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#extended-resources). Examples include "cpu", "memory", "ephemeral-storage", and hugepages.≤
array∫G
E
C#/definitions/io.k8s.api.core.v1.NodeAllocatableResourceClaimStatus˙4
x-kubernetes-list-map-keys- resourceClaimName
˙ 
x-kubernetes-list-typemap
˙4
x-kubernetes-patch-merge-keyresourceClaimName
˙'
x-kubernetes-patch-strategymerge

Õ
nominatedNodeName∑"©nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.≤
string
◊
observedGeneration¿int64"™If set, this represents the .metadata.generation that the pod status was set based upon. The PodObservedGenerationTracking feature gate must be enabled to use this field.≤	
integer
∆	
phaseº	"Æ	The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:

Pending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.

More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase≤
string
{
podIPr"epodIP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.≤
string
“
podIPs«"podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.≤
array∫*
(
&#/definitions/io.k8s.api.core.v1.PodIP˙%
x-kubernetes-list-map-keys- ip
˙ 
x-kubernetes-list-typemap
˙%
x-kubernetes-patch-merge-keyip
˙'
x-kubernetes-patch-strategymerge

ä
qosClass˝"ÔThe Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-qos/#quality-of-service-classes≤
string
v
reasonl"_A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'≤
string
´
resize†"íStatus of resources resize desired for pod's containers. It is empty if no resources resize is pending. Any changes to container resources will automatically set this to "Proposed" Deprecated: Resize status is moved to two pod conditions PodResizePending and PodResizeInProgress. PodResizePending will track states where the spec has been resized, but the Kubelet has not yet allocated the resources. PodResizeInProgress will track in-progress resizes, and should be present whenever allocated resources != acknowledged resources.≤
string
™
resourceClaimStatusesê"Status of resource claims.≤
array∫;
9
7#/definitions/io.k8s.api.core.v1.PodResourceClaimStatus˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙2
x-kubernetes-patch-strategymerge,retainKeys

Í
	resources‹
5#/definitions/io.k8s.api.core.v1.ResourceRequirements"¢Resources represents the compute resource requests and limits that have been applied at the pod level if pod-level requests or limits are set in PodSpec.Resources
€
	startTimeÕ
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"ëRFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
ô
volumeHealthà"xvolumeHealth contains node-reported health for each volume the pod is using. Populated by the kubelet on the pod's node.≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.PodVolumeHealth˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap

µ
"io.k8s.api.core.v1.PodVolumeHealthé"xPodVolumeHealth contains health information for a volume used by a pod, reported by the CSI node plugin via the kubelet.öname≤
object ˛
ù
healthConditionsà"êconditions is the set of adverse conditions reported by the CSI node plugin for this volume on this node. At most 16 conditions may be reported.≤
array∫:
8
6#/definitions/io.k8s.api.core.v1.VolumeHealthCondition˙2
x-kubernetes-list-map-keys- status
- reason
˙ 
x-kubernetes-list-typemap
˙)
x-kubernetes-patch-merge-key	status
˙'
x-kubernetes-patch-strategymerge

ö
lastTransitionTimeÉ
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"HlastTransitionTime is when the current set of conditions first appeared.
?
name7"*name matches an entry in pod.spec.volumes.≤
string
‡
io.k8s.api.core.v1.PortStatusæ";PortStatus represents the error condition of a service portöportöprotocol≤
object ‡
Ÿ
errorœ"¡Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
  CamelCase names
- cloud provider specific error values must have names that comply with the
  format foo.example.com/CamelCase.≤
string
i
portaint32"LPort is the port number of the service port of which status is recorded here≤	
integer
ñ
protocolâ"|Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"≤
string
†
'io.k8s.api.core.v1.PortworxVolumeSourceÙ";PortworxVolumeSource represents a Portworx volume resource.övolumeID≤
object ù
Õ
fsType¬"¥fSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.≤
string
Å
readOnlyu"greadOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
G
volumeID;".volumeID uniquely identifies a Portworx volume≤
string
Ç
*io.k8s.api.core.v1.PreferredSchedulingTerm”"ÆAn empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).öweightö
preference≤
object ˝
Ç

preferencet
1#/definitions/io.k8s.api.core.v1.NodeSelectorTerm"?A node selector term, associated with the corresponding weight.
v
weightlint32"WWeight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.≤	
integer
Â
io.k8s.api.core.v1.Probe»"ÄProbe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.≤
object ∂
l
execd
+#/definitions/io.k8s.api.core.v1.ExecAction"5Exec specifies a command to execute in the container.
ß
failureThresholdíint32"}Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.≤	
integer
`
grpcX
+#/definitions/io.k8s.api.core.v1.GRPCAction")GRPC specifies a GRPC HealthCheckRequest.
n
httpGetc
.#/definitions/io.k8s.api.core.v1.HTTPGetAction"1HTTPGet specifies an HTTP GET request to perform.
‚
initialDelaySeconds int32"¥Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes≤	
integer
}
periodSecondslint32"WHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.≤	
integer
Œ
successThresholdπint32"£Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.≤	
integer
p
	tcpSocketc
0#/definitions/io.k8s.api.core.v1.TCPSocketAction"/TCPSocket specifies a connection to a TCP port.
Ω
terminationGracePeriodSecondsõint64"ÖOptional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.≤	
integer
‚
timeoutSecondsœint32"πNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes≤	
integer
∏
(io.k8s.api.core.v1.ProjectedVolumeSourceã"$Represents a projected volume source≤
object ÷
‚
defaultMode“int32"ºdefaultMode are the mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.≤	
integer
ü
defaultUserèint64"˘defaultUser is Optional: The owner UID of the created files by default. The defaultUser field is only used as a fallback when the item-level user field is unset. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
Ã
sources¿"Vsources is the list of volume projections. Each entry in this list handles one source.≤
array∫5
3
1#/definitions/io.k8s.api.core.v1.VolumeProjection˙#
x-kubernetes-list-type	atomic

‚
&io.k8s.api.core.v1.QuobyteVolumeSource∑"áRepresents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.öregistryövolume≤
object ä
G
group>"1group to map volume access to Default is no group≤
string
Ç
readOnlyv"hreadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.≤	
boolean
·
registry‘"∆registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes≤
string
ú
tenantë"Étenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin≤
string
Q
userI"<user to map volume access to Defaults to serivceaccount user≤
string
d
volumeZ"Mvolume is a string that references an already created Quobyte volume by name.≤
string
”
"io.k8s.api.core.v1.RBDVolumeSource¨"àRepresents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.ömonitorsöimage≤
object ˇ
¡
fsType∂"®fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd≤
string
{
imager"eimage is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it≤
string
´
keyringü"ëkeyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it≤
string
¿
monitors≥"qmonitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

â
poolÄ"spool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it≤
string
¥
readOnlyß"òreadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it≤	
boolean
˘
	secretRefÎ
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"±secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
ã
userÇ"uuser is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it≤
string
Ó
 io.k8s.api.core.v1.ResourceClaim…"=ResourceClaim references one entry in PodSpec.ResourceClaims.öname≤
object Ù
±
name®"öName must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.≤
string
Ω
request±"£Request is the name chosen for a request in the referenced claim. If empty, everything from the claim is made available, otherwise only the result of this request.≤
string
Ì
(io.k8s.api.core.v1.ResourceFieldSelector¿"ZResourceFieldSelector represents container resources (cpu, memory) and their output formatöresource≤
object •
Y
containerNameH";Container name: required for volumes, optional for env vars≤
string
ê
divisorÑ
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity"ESpecifies the output format of the exposed resources, defaults to "1"
5
resource)"Required: resource to select≤
string˙"
x-kubernetes-map-type	atomic

Ã
!io.k8s.api.core.v1.ResourceHealth¶"èResourceHealth represents the health of a resource. It has the latest device health information. This is a part of KEP https://kep.k8s.io/4680.ö
resourceID≤
object ¯
Î
health‡"“Health of the resource. can be one of:
 - Healthy: operates as normal
 - Unhealthy: reported unhealthy. We consider this a temporary health issue
              since we do not have a mechanism today to distinguish
              temporary and permanent issues.
 - Unknown: The status cannot be determined.
            For example, Device Plugin got unregistered and hasn't been re-registered since.

In future we may want to introduce the PermanentlyUnhealthy Status.≤
string
à
message¸"ÓMessage provides human-readable context for Health (e.g. "ECC error count exceeded threshold"). This field is populated by the kubelet when ResourceHealthStatusMessage is enabled if the DRA plugin returns a message, and is null otherwise.≤
string
}

resourceIDo"bResourceID is the unique identifier of the resource. See the ResourceID type for more information.≤
string
£	
'io.k8s.api.core.v1.ResourceRequirements˜"AResourceRequirements describes the compute resource requirements.≤
object •
ˇ
claimsÙ"ÂClaims lists the names of resources, defined in spec.resourceClaims, that are used by this container.

This field depends on the DynamicResourceAllocation feature gate.

This field is immutable. It can only be set for containers.≤
array∫2
0
.#/definitions/io.k8s.api.core.v1.ResourceClaim˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap

ˆ
limitsÎ"õLimits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
ß
requestsö" Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
·

!io.k8s.api.core.v1.ResourceStatusª
"MResourceStatus represents the status of a single resource allocated to a Pod.öname≤
object ÷	
Ç
name˘"ÎName of the resource. Must be unique within the pod and in case of non-DRA resource, match one of the resources from the pod spec. For DRA resources, the value must be "claim:<claim_name>/<request>" when container.resources.claims[*].request is set or "claim:<claim_name>" when container.resources.claims[*].request is empty. For DRA-backed extended resources, "claim:<claim_name>/<request>" is used when the claim name and request name are recorded in pod.status.extendedResourceClaimStatus. When this status is reported about a container, the "claim_name" and "request" must match one of the claims of this container.≤
string
Œ
	resources¿"™List of unique resources health. Each element in the list contains an unique resource ID and its health. At a minimum, for the lifetime of a Pod, resource ID must uniquely identify the resource allocated to the Pod on the Node. If other Pod on the same Node reports the status with the same resource ID, it must be the same resource they share. See ResourceID type definition for a specific format it has in various use cases.≤
array∫3
1
/#/definitions/io.k8s.api.core.v1.ResourceHealth˙-
x-kubernetes-list-map-keys- resourceID
˙ 
x-kubernetes-list-typemap

º
!io.k8s.api.core.v1.SELinuxOptionsñ"<SELinuxOptions are the labels to be applied to the container≤
object …
Q
levelH";Level is SELinux level label that applies to the container.≤
string
P
roleH";Role is a SELinux role label that applies to the container.≤
string
P
typeH";Type is a SELinux type label that applies to the container.≤
string
P
userH";User is a SELinux user label that applies to the container.≤
string
—
&io.k8s.api.core.v1.ScaleIOVolumeSource¶":ScaleIOVolumeSource represents a persistent ScaleIO volumeögatewayösystemö	secretRef≤
object º

Ø
fsType§"ñfsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".≤
string
O
gatewayD"7gateway is the host address of the ScaleIO API Gateway.≤
string
z
protectionDomainf"YprotectionDomain is the name of the ScaleIO Protection Domain for the configured storage.≤
string
Å
readOnlyu"greadOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
–
	secretRef¬
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"àsecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
h

sslEnabledZ"LsslEnabled Flag enable/disable SSL communication with Gateway, default false≤	
boolean
ü
storageModeè"ÅstorageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.≤
string
j
storagePool["NstoragePool is the ScaleIO Storage Pool associated with the protection domain.≤
string
Y
systemO"Bsystem is the name of the storage system as configured in ScaleIO.≤
string
ê

volumeNameÅ"tvolumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.≤
string
ã
!io.k8s.api.core.v1.SeccompProfileÂ"fSeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.ötype≤
object Ú
Œ
localhostProfileπ"´localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must be set if type is "Localhost". Must NOT be set for any other type.≤
string
û
typeï"átype indicates which kind of seccomp profile will be applied. Valid options are:

Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.≤
string˙r
x-kubernetes-unions[Y- discriminator: type
  fields-to-discriminateBy:
    localhostProfile: LocalhostProfile

ˇ
io.k8s.api.core.v1.Secret·"ÇSecret holds secret data of a certain type. The total bytes of the values in the Data field must be less than MaxSecretSize bytes.≤
object ¸
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
Œ
data≈"°Data contains the secret data. Each key must consist of alphanumeric characters, '-', '_' or '.'. The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here. Described in https://tools.ietf.org/html/rfc4648#section-4™
byte≤
string≤
object
Â
	immutable◊"»Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.≤	
boolean
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
 

stringDataª"ùstringData allows specifying non-binary secret data in string form. It is provided as a write-only input field for convenience. All keys and values are merged into the data field on write, overwriting any existing values. The stringData field is never output when reading from the API.™
≤
string≤
object
°
typeò"äUsed to facilitate programmatic handling of secret data. More info: https://kubernetes.io/docs/concepts/configuration/secret/#secret-types≤
string˙N
x-kubernetes-group-version-kind+)- group: ""
  kind: Secret
  version: v1

Ï
"io.k8s.api.core.v1.SecretEnvSource≈"πSecretEnvSource selects a Secret to populate the environment variables with.

The contents of the target Secret's Data field will represent the key-value pairs as environment variables.≤
object ˙
±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
D
optional8"*Specify whether the Secret must be defined≤	
boolean
Ó
$io.k8s.api.core.v1.SecretKeySelector≈",SecretKeySelector selects a key of a Secret.ökey≤
object ›
V
keyO"BThe key of the secret to select from.  Must be a valid secret key.≤
string
±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
O
optionalC"5Specify whether the Secret or its key must be defined≤	
boolean˙"
x-kubernetes-map-type	atomic

´	
io.k8s.api.core.v1.SecretListâ	"SecretList is a list of Secret.öitems≤
object ¸
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
™
items†"fItems is a list of secret objects. More info: https://kubernetes.io/docs/concepts/configuration/secret≤
array∫+
)
'#/definitions/io.k8s.api.core.v1.Secret
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙R
x-kubernetes-group-version-kind/-- group: ""
  kind: SecretList
  version: v1

Æ

#io.k8s.api.core.v1.SecretProjectionÜ
"çAdapts a secret into a projected volume.

The contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.≤
object Á
–
items∆"‚items if unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.KeyToPath˙#
x-kubernetes-list-type	atomic

±
name®"öName of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
^
optionalR"Doptional field specify whether the Secret or its key must be defined≤	
boolean
§
%io.k8s.api.core.v1.SecretVolumeSource˙"ÌAdapts a Secret into a volume.

The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.≤
object ˚
˘
defaultModeÈint32"”defaultMode is Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.≤	
integer
ü
defaultUserèint64"˘defaultUser is Optional: The owner UID of the created files by default. The defaultUser field is only used as a fallback when the item-level user field is unset. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
–
items∆"‚items If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.≤
array∫.
,
*#/definitions/io.k8s.api.core.v1.KeyToPath˙#
x-kubernetes-list-type	atomic

_
optionalS"Eoptional field specify whether the Secret or its keys must be defined≤	
boolean
¶

secretNameó"âsecretName is the name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret≤
string
„ 
"io.k8s.api.core.v1.SecurityContextº "‹SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.≤
object Œ
ñ
allowPrivilegeEscalation˘"ÍAllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.≤	
boolean
Ö
appArmorProfileÒ
0#/definitions/io.k8s.api.core.v1.AppArmorProfile"ºappArmorProfile is the AppArmor options to use by this container. If set, this profile overrides the pod's appArmorProfile. Note that this field cannot be set when spec.os.name is windows.
á
capabilitiesˆ
-#/definitions/io.k8s.api.core.v1.Capabilities"ƒThe capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
Ê

privileged◊"»Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.≤	
boolean
å
	procMount˛"procMount denotes the type of proc mount to use for the containers. The default value is Default which uses the container runtime defaults for readonly paths and masked paths. Note that this field cannot be set when spec.os.name is windows.≤
string
¥
readOnlyRootFilesystemô"äWhether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.≤	
boolean
‘

runAsGroup≈int64"ØThe GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.≤	
integer
Ω
runAsNonRoot¨"ùIndicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.≤	
boolean
Ò
	runAsUser„int64"ÕThe UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.≤	
integer
µ
seLinuxOptions¢
/#/definitions/io.k8s.api.core.v1.SELinuxOptions"ÓThe SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
®
seccompProfileï
/#/definitions/io.k8s.api.core.v1.SeccompProfile"·The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
É
windowsOptions
>#/definitions/io.k8s.api.core.v1.WindowsSecurityContextOptions"≠The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
Ö
io.k8s.api.core.v1.ServiceÊ"ÁService is a named abstraction of software service (for example, mysql) consisting of local port (for example 3306) that the proxy listens on, and the selector that determines which pods will answer requests sent through the proxy.≤
object õ

æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
≈
specº
,#/definitions/io.k8s.api.core.v1.ServiceSpec"ãSpec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
˛
statusÛ
.#/definitions/io.k8s.api.core.v1.ServiceStatus"¿Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status˙O
x-kubernetes-group-version-kind,*- group: ""
  kind: Service
  version: v1

π
!io.k8s.api.core.v1.ServiceAccountì"∫ServiceAccount binds together: * a name, understood by users, and perhaps by peripheral systems, for an identity * a principal that can be authenticated and authorized * a set of secrets≤
object Ó
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
◊
automountServiceAccountToken∂"ßAutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.≤	
boolean
ã
imagePullSecretsˆ"áImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod≤
array∫9
7
5#/definitions/io.k8s.api.core.v1.LocalObjectReference˙#
x-kubernetes-list-type	atomic

π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
“
metadata≈
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"ÉStandard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
±
secrets•"¿Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a "kubernetes.io/enforce-mountable-secrets" annotation set to "true". The "kubernetes.io/enforce-mountable-secrets" annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to mounted secrets. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created. More info: https://kubernetes.io/docs/concepts/configuration/secret≤
array∫4
2
0#/definitions/io.k8s.api.core.v1.ObjectReference˙'
x-kubernetes-list-map-keys	- name
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyname
˙'
x-kubernetes-patch-strategymerge
˙V
x-kubernetes-group-version-kind31- group: ""
  kind: ServiceAccount
  version: v1

Î	
%io.k8s.api.core.v1.ServiceAccountList¡	"6ServiceAccountList is a list of ServiceAccount objectsöitems≤
object ï
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
√
itemsπ"wList of ServiceAccounts. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/≤
array∫3
1
/#/definitions/io.k8s.api.core.v1.ServiceAccount
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙Z
x-kubernetes-group-version-kind75- group: ""
  kind: ServiceAccountList
  version: v1

Ó

0io.k8s.api.core.v1.ServiceAccountTokenProjectionπ
"ÏServiceAccountTokenProjection represents a projected service account token volume. This projection can be used to insert a service account token into the pods runtime filesystem for use against APIs (Kubernetes API Server or otherwise).öpath≤
object ¥
í
audienceÖ"˜audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.≤
string
π
expirationSeconds£int64"çexpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.≤	
integer
h
path`"Spath is the path relative to the mount point of the file to project the token into.≤
string
ˆ
userÌint64"◊user is Optional: The owner UID of the created file. If specified, the item-level user field takes precedence over defaultUser. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.≤	
integer
‹
io.k8s.api.core.v1.ServiceListπ"%ServiceList holds a list of services.öitems≤
object •
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
T
itemsK"List of services≤
array∫,
*
(#/definitions/io.k8s.api.core.v1.Service
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds˙S
x-kubernetes-group-version-kind0.- group: ""
  kind: ServiceList
  version: v1

“
io.k8s.api.core.v1.ServicePortØ"3ServicePort contains information on service's port.öport≤
object ‰
â
appProtocol˘"ÎThe application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:

* Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and https://www.iana.org/assignments/service-names).

* Kubernetes-defined prefixed names:
  * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
  * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
  * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455

* Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.≤
string
≤
name©"õThe name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.≤
string
Ç
nodePortıint32"ﬂThe port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport≤	
integer
K
portCint32".The port that will be exposed by this service.≤	
integer
j
protocol^"QThe IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.≤
string
‚

targetPort”
=#/definitions/io.k8s.apimachinery.pkg.util.intstr.IntOrString"ëNumber or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
ºh
io.k8s.api.core.v1.ServiceSpecôh"FServiceSpec describes the attributes that a user creates on a service.≤
object ¬g
ã
allocateLoadBalancerNodePortsÈ"⁄allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.≤	
boolean
·
	clusterIP”"≈clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address. Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies≤
string
¬

clusterIPs≥"ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address.  Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs[0] and clusterIP have the same value.

This field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ò
externalIPsà"≈externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

∏
externalNameß"ôexternalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be "ExternalName".≤
string
◊
externalTrafficPolicyΩ"ØexternalTrafficPolicy describes how nodes distribute service traffic they receive on one of the Service's "externally-facing" addresses (NodePorts, ExternalIPs, and LoadBalancer IPs). If set to "Local", the proxy will configure the service in a way that assumes that external load balancers will take care of balancing the service traffic between nodes, and so each node will deliver traffic only to the node-local endpoints of the service, without masquerading the client source IP. (Traffic mistakenly sent to a node with no endpoints will be dropped.) The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features). Note that traffic sent to an External IP or LoadBalancer IP from within the cluster will always get "Cluster" semantics, but clients sending to a NodePort from within the cluster may need to take traffic policy into account when picking a node.≤
string
º
healthCheckNodePort§int32"éhealthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type). This field cannot be updated once set.≤	
integer
œ
internalTrafficPolicyµ"ßInternalTrafficPolicy describes how nodes distribute service traffic they receive on the ClusterIP. If set to "Local", the proxy will assume that pods only want to talk to endpoints of the service on the same node as the pod, dropping the traffic if there are no local endpoints. The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features).≤
string
ª

ipFamilies¨"ÈIPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are "IPv4" and "IPv6".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to "headless" services. This field will be wiped when updating a Service to type ExternalName.

This field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

‘
ipFamilyPolicy¡"≥IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be "SingleStack" (a single IP family), "PreferDualStack" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or "RequireDualStack" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.≤
string
◊
loadBalancerClass¡"≥loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. "internal-vip" or "example.com/internal-vip". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.≤
string
á
loadBalancerIPÙ"ÊOnly applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations. Using it is non-portable and it may not support dual-stack. Users are encouraged to use implementation-specific annotations when available.≤
string
µ
loadBalancerSourceRangesò"’If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ï
portsã"üThe list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies≤
array∫0
.
,#/definitions/io.k8s.api.core.v1.ServicePort˙2
x-kubernetes-list-map-keys- port
- protocol
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keyport
˙'
x-kubernetes-patch-strategymerge

¨
publishNotReadyAddressesè"ÄpublishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered "ready" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.≤	
boolean
–
selector√"ÄRoute service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/™
≤
string≤
object˙"
x-kubernetes-map-type	atomic

•
sessionAffinityë"ÉSupports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies≤
string
ö
sessionAffinityConfigÄ
6#/definitions/io.k8s.api.core.v1.SessionAffinityConfig"FsessionAffinityConfig contains the configurations of session affinity.
©
trafficDistributionë"ÉTrafficDistribution offers a way to express preferences for how traffic is distributed to Service endpoints. Implementations can use this field as a hint, but are not required to guarantee strict adherence. If the field is not set, the implementation will apply its default routing strategy. If set to "PreferClose", implementations should prioritize endpoints that are in the same zone.≤
string
ã
typeÇ"Ùtype determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. "ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types≤
string
¢
 io.k8s.api.core.v1.ServiceStatus˝"9ServiceStatus represents the current status of a service.≤
object ≥
î

conditionsÖ"Current service state≤
array∫@
>
<#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Condition˙'
x-kubernetes-list-map-keys	- type
˙ 
x-kubernetes-list-typemap
˙'
x-kubernetes-patch-merge-keytype
˙'
x-kubernetes-patch-strategymerge

ô
loadBalancerà
3#/definitions/io.k8s.api.core.v1.LoadBalancerStatus"QLoadBalancer contains the current status of the load-balancer, if one is present.
ë
(io.k8s.api.core.v1.SessionAffinityConfig‰"HSessionAffinityConfig represents the configurations of session affinity.≤
object ã
à
clientIP|
/#/definitions/io.k8s.api.core.v1.ClientIPConfig"IclientIP contains the configurations of Client IP based session affinity.
∞
io.k8s.api.core.v1.SleepActionç"'SleepAction describes a "sleep" action.öseconds≤
object L
J
seconds?int64"*Seconds is the number of seconds to sleep.≤	
integer
¯	
(io.k8s.api.core.v1.StorageOSVolumeSourceÀ	"2Represents a StorageOS persistent volume resource.≤
object à	
Œ
fsType√"µfsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.≤
string
Å
readOnlyu"greadOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.≤	
boolean
œ
	secretRef¡
5#/definitions/io.k8s.api.core.v1.LocalObjectReference"ásecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
ã

volumeName}"pvolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.≤
string
–
volumeNamespaceº"ÆvolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.≤
string
 
io.k8s.api.core.v1.Sysctl¨"+Sysctl defines a kernel parameter to be setönameövalue≤
object b
.
name&"Name of a property to set≤
string
0
value'"Value of a property to set≤
string
ó
"io.k8s.api.core.v1.TCPSocketAction"=TCPSocketAction describes an action based on opening a socketöport≤
object õ
O
hostG":Optional: Host name to connect to, defaults to the pod IP.≤
string
«
portæ
=#/definitions/io.k8s.apimachinery.pkg.util.intstr.IntOrString"}Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
”
io.k8s.api.core.v1.Taint∂"`The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.ökeyöeffect≤
object ∂
†
effectï"áRequired. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.≤
string
D
key="0Required. The taint key to be applied to a node.≤
string
É
	timeAddedv
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time";TimeAdded represents the time at which the taint was added.
E
value<"/The taint value corresponding to the taint key.≤
string
≈
io.k8s.api.core.v1.Toleration£"çThe pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.≤
object Ñ

∏
effect≠"üEffect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.≤
string
”
keyÀ"ΩKey is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.≤
string
·
operator‘"∆Operator represents a key's relationship to the value. Valid operators are Exists, Equal, Lt, and Gt. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category. Lt and Gt perform numeric comparisons (requires feature gate TaintTolerationComparisonOperators).≤
string
Á
tolerationSeconds—int64"ªTolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.≤	
integer
¢
valueò"äValue is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.≤
string
æ.
+io.k8s.api.core.v1.TopologySpreadConstrainté."XTopologySpreadConstraint specifies how to spread matching pods among the given topology.ömaxSkewötopologyKeyöwhenUnsatisfiable≤
object ˘,
˚
labelSelectorÈ
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector"§LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
î
matchLabelKeysÅ"æMatchLabelKeys is a set of pod label keys to select the pods over which spreading will be calculated. The keys are used to lookup values from the incoming pod labels, those key-value labels are ANDed with labelSelector to select the group of existing pods over which spreading will be calculated for the incoming pod. The same key is forbidden to exist in both MatchLabelKeys and LabelSelector. MatchLabelKeys cannot be set when LabelSelector isn't set. Keys that don't exist in the incoming pod labels will be ignored. A null or empty list means only match against labelSelector.

This is a beta field and requires the MatchLabelKeysInPodTopologySpread feature gate to be enabled (enabled by default).≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

Ü
maxSkew˙int32"‰MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. The global minimum is the minimum number of matching pods in an eligible domain or zero if the number of eligible domains is less than MinDomains. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 2/2/1: In this case, the global minimum is 1. | zone1 | zone2 | zone3 | |  P P  |  P P  |   P   | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 2/2/2; scheduling it onto zone1(zone2) would make the ActualSkew(3-1) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.≤	
integer
µ	

minDomains¶	int32"ê	MinDomains indicates a minimum number of eligible domains. When the number of eligible domains with matching topology keys is less than minDomains, Pod Topology Spread treats "global minimum" as 0, and then the calculation of Skew is performed. And when the number of eligible domains with matching topology keys equals or greater than minDomains, this value has no effect on scheduling. As a result, when the number of eligible domains is less than minDomains, scheduler won't schedule more than maxSkew Pods to those domains. If value is nil, the constraint behaves as if MinDomains is equal to 1. Valid values are integers greater than 0. When value is not nil, WhenUnsatisfiable must be DoNotSchedule.

For example, in a 3-zone cluster, MaxSkew is set to 2, MinDomains is set to 5 and pods with the same labelSelector spread as 2/2/2: | zone1 | zone2 | zone3 | |  P P  |  P P  |  P P  | The number of domains is less than 5(MinDomains), so "global minimum" is treated as 0. In this situation, new pod with the same labelSelector cannot be scheduled, because computed skew will be 3(3 - 0) if new Pod is scheduled to any of the three zones, it will violate MaxSkew.≤	
integer
®
nodeAffinityPolicyë"ÉNodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector when calculating pod topology spread skew. Options are: - Honor: only nodes matching nodeAffinity/nodeSelector are included in the calculations. - Ignore: nodeAffinity/nodeSelector are ignored. All nodes are included in the calculations.

If this value is nil, the behavior is equivalent to the Honor policy.≤
string
â
nodeTaintsPolicyÙ"ÊNodeTaintsPolicy indicates how we will treat node taints when calculating pod topology spread skew. Options are: - Honor: nodes without taints, along with tainted nodes for which the incoming pod has a toleration, are included. - Ignore: node taints are ignored. All nodes are included.

If this value is nil, the behavior is equivalent to the Ignore policy.≤
string
ñ
topologyKeyÜ"¯TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a "bucket", and try to put balanced number of pods into each bucket. We define a domain as a particular instance of a topology. Also, we define an eligible domain as a domain whose nodes meet the requirements of nodeAffinityPolicy and nodeTaintsPolicy. e.g. If TopologyKey is "kubernetes.io/hostname", each Node is a domain of that topology. And, if TopologyKey is "topology.kubernetes.io/zone", each zone is a domain of that topology. It's a required field.≤
string
–
whenUnsatisfiable∫"¨WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location,
  but giving higher precedence to topologies that would help reduce the
  skew.
A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.≤
string
ÿ
,io.k8s.api.core.v1.TypedLocalObjectReferenceß"~TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace.ökindöname≤
object Â
⁄
apiGroupÕ"øAPIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.≤
string
B
kind:"-Kind is the type of resource being referenced≤
string
B
name:"-Name is the name of resource being referenced≤
string˙"
x-kubernetes-map-type	atomic

´
'io.k8s.api.core.v1.TypedObjectReferenceˇ"^TypedObjectReference contains enough information to let you locate the typed referenced objectökindöname≤
object Ç
⁄
apiGroupÕ"øAPIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.≤
string
B
kind:"-Kind is the type of resource being referenced≤
string
B
name:"-Name is the name of resource being referenced≤
string
ö
	namespaceå"˛Namespace is the namespace of resource being referenced Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. (Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.≤
string
¯R
io.k8s.api.core.v1.Volume⁄R"[Volume represents a named volume in a pod that may be accessed by any container in the pod.öname≤
object ÁQ
Ã
awsElasticBlockStore≥
A#/definitions/io.k8s.api.core.v1.AWSElasticBlockStoreVolumeSource"ÌawsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
•
	azureDiskó
6#/definitions/io.k8s.api.core.v1.AzureDiskVolumeSource"‹azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod. Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type are redirected to the disk.csi.azure.com CSI driver.
®
	azureFileö
6#/definitions/io.k8s.api.core.v1.AzureFileVolumeSource"ﬂazureFile represents an Azure File Service mount on the host and bind mount to the pod. Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type are redirected to the file.csi.azure.com CSI driver.
„
cephfsÿ
3#/definitions/io.k8s.api.core.v1.CephFSVolumeSource"†cephFS represents a Ceph FS mount on the host that shares a pod's lifetime. Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
ÿ
cinderÕ
3#/definitions/io.k8s.api.core.v1.CinderVolumeSource"ïcinder represents a cinder volume attached and mounted on kubelets host machine. Deprecated: Cinder is deprecated. All operations for the in-tree cinder type are redirected to the cinder.csi.openstack.org CSI driver. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
à
	configMap{
6#/definitions/io.k8s.api.core.v1.ConfigMapVolumeSource"AconfigMap represents a configMap that should populate this volume
´
csi£
0#/definitions/io.k8s.api.core.v1.CSIVolumeSource"ocsi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers.
û
downwardAPIé
8#/definitions/io.k8s.api.core.v1.DownwardAPIVolumeSource"RdownwardAPI represents downward API about the pod that should populate this volume
÷
emptyDir…
5#/definitions/io.k8s.api.core.v1.EmptyDirVolumeSource"èemptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
◊
	ephemeral…
6#/definitions/io.k8s.api.core.v1.EphemeralVolumeSource"éephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.

Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity
   tracking are needed,
c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through
   a PersistentVolumeClaim (see EphemeralVolumeSource for more
   information on the connection between this volume type
   and PersistentVolumeClaim).

Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.

Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.

A pod can use both types of ephemeral volumes and persistent volumes at the same time.
™
fc£
/#/definitions/io.k8s.api.core.v1.FCVolumeSource"pfc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
˜

flexVolumeË
1#/definitions/io.k8s.api.core.v1.FlexVolumeSource"≤flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
†
flockerî
4#/definitions/io.k8s.api.core.v1.FlockerVolumeSource"€flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running. Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
ø
gcePersistentDisk©
>#/definitions/io.k8s.api.core.v1.GCEPersistentDiskVolumeSource"ÊgcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
…
gitRepoΩ
4#/definitions/io.k8s.api.core.v1.GitRepoVolumeSource"ÑgitRepo represents a git repository at a particular revision. Deprecated: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
Ù
	glusterfsÊ
6#/definitions/io.k8s.api.core.v1.GlusterfsVolumeSource"´glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported.
î
hostPathá
5#/definitions/io.k8s.api.core.v1.HostPathVolumeSource"ÕhostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
á
image˝
2#/definitions/io.k8s.api.core.v1.ImageVolumeSource"∆image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine. The volume is resolved at pod startup depending on which PullPolicy value is provided:

- Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. - Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. - IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.

The volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation. A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field. The OCI object gets mounted in a single directory (spec.containers[*].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images. The volume will be mounted read-only (ro). Sub path mounts for containers are not supported (spec.containers[*].volumeMounts.subpath) before 1.33. The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.
¯
iscsiÓ
2#/definitions/io.k8s.api.core.v1.ISCSIVolumeSource"∑iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes/#iscsi
≥
name™"úname of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names≤
string
ƒ
nfsº
0#/definitions/io.k8s.api.core.v1.NFSVolumeSource"ánfs represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
¨
persistentVolumeClaimí
B#/definitions/io.k8s.api.core.v1.PersistentVolumeClaimVolumeSource"ÀpersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
¡
photonPersistentDisk®
A#/definitions/io.k8s.api.core.v1.PhotonPersistentDiskVolumeSource"‚photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine. Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
∑
portworxVolume§
5#/definitions/io.k8s.api.core.v1.PortworxVolumeSource"ÍportworxVolume represents a portworx volume attached and mounted on kubelets host machine. Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type are redirected to the pxd.portworx.com CSI driver.
ñ
	projectedà
6#/definitions/io.k8s.api.core.v1.ProjectedVolumeSource"Nprojected items for all in one resources secrets, configmaps, and downward API
Ë
quobyte‹
4#/definitions/io.k8s.api.core.v1.QuobyteVolumeSource"£quobyte represents a Quobyte mount on the host that shares a pod's lifetime. Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
ﬂ
rbd◊
0#/definitions/io.k8s.api.core.v1.RBDVolumeSource"¢rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported.
Ù
scaleIOË
4#/definitions/io.k8s.api.core.v1.ScaleIOVolumeSource"ØscaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes. Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
≈
secret∫
3#/definitions/io.k8s.api.core.v1.SecretVolumeSource"Çsecret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
ı
	storageosÁ
6#/definitions/io.k8s.api.core.v1.StorageOSVolumeSource"¨storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes. Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported.
¬
vsphereVolume∞
?#/definitions/io.k8s.api.core.v1.VsphereVirtualDiskVolumeSource"ÏvsphereVolume represents a vSphere volume attached and mounted on kubelets host machine. Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type are redirected to the csi.vsphere.vmware.com CSI driver.
÷
io.k8s.api.core.v1.VolumeDevice≤"JvolumeDevice describes a mapping of a raw block device within a container.önameö
devicePath≤
object √
l

devicePath^"QdevicePath is the path inside of the container that the device will be mapped to.≤
string
S
nameK">name must match the name of a persistentVolumeClaim in the pod≤
string
Ó
(io.k8s.api.core.v1.VolumeHealthCondition¡"SVolumeHealthCondition represents an adverse health condition reported for a volume.östatusöreason≤
object À
u
messagej"]message is a human-readable description. Maximum permitted length of a message is 1024 bytes.≤
string
…
reasonæ"∞reason is a brief CamelCase machine-parseable reason. Together with status it forms the unique identity of a condition entry. Maximum permitted length of a reason is 256 bytes.≤
string
Ö
status˙"Ïstatus is the machine-parseable health category. Possible values: - "Inaccessible": the volume cannot be accessed. - "DataLoss": data loss has been detected on the volume. - "Degraded": the volume is functioning with reduced capability.≤
string
¬
io.k8s.api.core.v1.VolumeMountü"@VolumeMount describes a mounting of a Volume within a container.önameö	mountPath≤
object ª
¿
bindMountOptions´"ÎbindMountOptions is the list of additional bind mount options to apply when mounting this volume into the container. Allowed values are noexec, nodev, and nosuid. These are Linux mount options and have no effect on Windows nodes. This field is not supported with image volumes. This is an alpha field and requires enabling the VolumeBindMountOptions feature gate.≤
array∫
≤
string˙ 
x-kubernetes-list-typeset

Z
	mountPathM"@Path within the container at which the volume should be mounted.≤
string
◊
mountPropagation¬"¥mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10. When RecursiveReadOnly is set to IfPossible or to Enabled, MountPropagation must be None or unspecified (which defaults to None).≤
string
:
name2"%This must match the Name of a Volume.≤
string
t
readOnlyh"ZMounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.≤	
boolean
≈
recursiveReadOnlyØ"°RecursiveReadOnly specifies whether read-only mounts should be handled recursively.

If ReadOnly is false, this field has no meaning and must be unspecified.

If ReadOnly is true, and this field is set to Disabled, the mount is not made recursively read-only.  If this field is set to IfPossible, the mount is made recursively read-only, if it is supported by the container runtime.  If this field is set to Enabled, the mount is made recursively read-only if it is supported by the container runtime, otherwise the pod will not be started and an error will be generated to indicate the reason.

If this field is set to IfPossible or Enabled, MountPropagation must be set to None (or be unspecified, which defaults to None).

If this field is not specified, it is treated as an equivalent of Disabled.≤
string
É
subPathx"kPath within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).≤
string
ø
subPathExprØ"°Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.≤
string
˜
$io.k8s.api.core.v1.VolumeMountStatusŒ"0VolumeMountStatus shows status of volume mounts.önameö	mountPath≤
object ˙
L
	mountPath?"2MountPath corresponds to the original VolumeMount.≤
string
N
nameF"9Name corresponds to the name of the original VolumeMount.≤
string
K
readOnly?"1ReadOnly corresponds to the original VolumeMount.≤	
boolean
˚
recursiveReadOnlyÂ"◊RecursiveReadOnly must be set to Disabled, Enabled, or unspecified (for non-readonly mounts). An IfPossible value in the original VolumeMount must be translated to Disabled or Enabled, depending on the mount result.≤
string
é
volumeStatus~
-#/definitions/io.k8s.api.core.v1.VolumeStatus"MvolumeStatus represents volume-type-specific status about the mounted volume.
ä
#io.k8s.api.core.v1.VolumeProjection‚"rProjection that may be projected along with other supported volume types. Exactly one of these fields must be set.≤
object ﬂ
 
clusterTrustBundle≥
=#/definitions/io.k8s.api.core.v1.ClusterTrustBundleProjection"ÒClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.

Alpha, gated by the ClusterTrustBundleProjection feature gate.

ClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.

Kubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.
~
	configMapq
4#/definitions/io.k8s.api.core.v1.ConfigMapProjection"9configMap information about the configMap data to project
Ü
downwardAPIw
6#/definitions/io.k8s.api.core.v1.DownwardAPIProjection"=downwardAPI information about the downwardAPI data to project
Â
podCertificate“
9#/definitions/io.k8s.api.core.v1.PodCertificateProjection"îProjects an auto-rotating credential bundle (private key and certificate chain) that the pod can use either as a TLS client or server.

Kubelet generates a private key and uses it to send a PodCertificateRequest to the named signer.  Once the signer approves the request and issues a certificate chain, Kubelet writes the key and certificate chain to the pod filesystem.  The pod does not start until certificates have been issued for each podCertificate projected volume source in its spec.

Kubelet will begin trying to rotate the certificate at the time indicated by the signer using the PodCertificateRequest.Status.BeginRefreshAt timestamp.

Kubelet can write a single file, indicated by the credentialBundlePath field, or separate files, indicated by the keyPath and certificateChainPath fields.

The credential bundle is a single file in PEM format.  The first PEM entry is the private key (in PKCS#8 format), and the remaining PEM entries are the certificate chain issued by the signer (typically, signers will return their certificate chain in leaf-to-root order).

Prefer using the credential bundle format, since your application code can read it atomically.  If you use keyPath and certificateChainPath, your application must make two separate file reads. If these coincide with a certificate rotation, it is possible that the private key and leaf certificate you read may not correspond to each other.  Your application will need to check for this condition, and re-read until they are consistent.

The named signer controls chooses the format of the certificate it issues; consult the signer implementation's documentation to learn how to use the certificates it issues.
r
secreth
1#/definitions/io.k8s.api.core.v1.SecretProjection"3secret information about the secret data to project
™
serviceAccountTokení
>#/definitions/io.k8s.api.core.v1.ServiceAccountTokenProjection"PserviceAccountToken is information about the serviceAccountToken data to project
∫
-io.k8s.api.core.v1.VolumeResourceRequirementsà"TVolumeResourceRequirements describes the storage resource requirements for a volume.≤
object £
ˆ
limitsÎ"õLimits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
ß
requestsö" Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/™?
=
;#/definitions/io.k8s.apimachinery.pkg.api.resource.Quantity≤
object
Õ
io.k8s.api.core.v1.VolumeStatus©"eVolumeStatus represents the status of a mounted volume. At most one of its members must be specified.≤
object ≥
∞
image¶
2#/definitions/io.k8s.api.core.v1.ImageVolumeStatus"pimage represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.
†
1io.k8s.api.core.v1.VsphereVirtualDiskVolumeSourceÍ"%Represents a vSphere volume resource.ö
volumePath≤
object ß
 
fsTypeø"±fsType is filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.≤
string
è
storagePolicyID|"ostoragePolicyID is the storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.≤
string
o
storagePolicyNameZ"MstoragePolicyName is the storage Policy Based Management (SPBM) profile name.≤
string
U

volumePathG":volumePath is the path that identifies vSphere volume vmdk≤
string
⁄
*io.k8s.api.core.v1.WeightedPodAffinityTerm´"vThe weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)öweightöpodAffinityTerm≤
object â
è
podAffinityTerm|
0#/definitions/io.k8s.api.core.v1.PodAffinityTerm"HRequired. A pod affinity term, associated with the corresponding weight.
u
weightkint32"Vweight associated with matching the corresponding podAffinityTerm, in the range 1-100.≤	
integer
ö	
0io.k8s.api.core.v1.WindowsSecurityContextOptionsÂ"OWindowsSecurityContextOptions contain Windows-specific options and credentials.≤
object Ö
È
gmsaCredentialSpec“"ƒGMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.≤
string
m
gmsaCredentialSpecNameS"FGMSACredentialSpecName is the name of the GMSA credential spec to use.≤
string
Â
hostProcess’"∆HostProcess determines if a container should be run as a 'Host Process' container. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers). In addition, if HostProcess is true then HostNetwork must also be set to true.≤	
boolean
ø
runAsUserName≠"üThe UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.≤
string
–
 io.k8s.api.discovery.v1.Endpoint´"FEndpoint represents a single logical "backend" implementing a service.ö	addresses≤
object »
ñ
	addressesà"»addresses of this endpoint. For EndpointSlices of addressType "IPv4" or "IPv6", the values are IP addresses in canonical form. The syntax and semantics of other addressType values are not defined. This must contain at least one address but no more than 100. EndpointSlices generated by the EndpointSlice controller will always have exactly 1 address. No semantics are defined for additional addresses beyond the first, and kube-proxy does not look at them.≤
array∫
≤
string˙ 
x-kubernetes-list-typeset

î

conditionsÖ
8#/definitions/io.k8s.api.discovery.v1.EndpointConditions"Iconditions contains information about the current status of the endpoint.
æ
deprecatedTopologyß"âdeprecatedTopology contains topology information part of the v1beta1 API. This field is deprecated, and will be removed when the v1beta1 API is removed (no sooner than kubernetes v1.24).  While this field can hold values, it is not writable through the v1 API, and any attempts to write to it will be silently ignored. Topology information can be found in the zone and nodeName fields instead.™
≤
string≤
object
è
hintsÖ
3#/definitions/io.k8s.api.discovery.v1.EndpointHints"Nhints contains information associated with how an endpoint should be consumed.
Œ
hostname¡"≥hostname of this endpoint. This field may be used by consumers of endpoints to distinguish endpoints from each other (e.g. in DNS names). Multiple endpoints which use the same hostname should be considered fungible (e.g. multiple A values in DNS). Must be lowercase and pass DNS Label (RFC 1123) validation.≤
string
í
nodeNameÖ"xnodeName represents the name of the Node hosting this endpoint. This can be used to determine endpoints local to a Node.≤
string
ê
	targetRefÇ
0#/definitions/io.k8s.api.core.v1.ObjectReference"NtargetRef is a reference to a Kubernetes object that represents this endpoint.
J
zoneB"5zone is the name of the Zone this endpoint exists in.≤
string
ﬁ
*io.k8s.api.discovery.v1.EndpointConditionsØ"CEndpointConditions represents the current condition of an endpoint.≤
object €
ç
readyÉ"Ùready indicates that this endpoint is ready to receive traffic, according to whatever system is managing the endpoint. A nil value should be interpreted as "true". In general, an endpoint should be marked ready if it is serving and not terminating, though this can be overridden in some cases, such as when the associated Service has set the publishNotReadyAddresses flag.≤	
boolean
¬
serving∂"ßserving indicates that this endpoint is able to receive traffic, according to whatever system is managing the endpoint. For endpoints backed by pods, the EndpointSlice controller will mark the endpoint as serving if the pod's Ready condition is True. A nil value should be interpreted as "true".≤	
boolean
É
terminatingt"fterminating indicates that this endpoint is terminating. A nil value should be interpreted as "false".≤	
boolean
Ü
%io.k8s.api.discovery.v1.EndpointHints‹"KEndpointHints provides hints describing how an endpoint should be consumed.≤
object Ä
˝
forNodes"âforNodes indicates the node(s) this endpoint should be consumed by when using topology aware routing. May contain a maximum of 8 entries.≤
array∫1
/
-#/definitions/io.k8s.api.discovery.v1.ForNode˙#
x-kubernetes-list-type	atomic

˝
forZones"âforZones indicates the zone(s) this endpoint should be consumed by when using topology aware routing. May contain a maximum of 8 entries.≤
array∫1
/
-#/definitions/io.k8s.api.discovery.v1.ForZone˙#
x-kubernetes-list-type	atomic

Í
$io.k8s.api.discovery.v1.EndpointPort¡"7EndpointPort represents a Port used by an EndpointSlice≤
object ‘
â
appProtocol˘"ÎThe application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:

* Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and https://www.iana.org/assignments/service-names).

* Kubernetes-defined prefixed names:
  * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
  * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
  * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455

* Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.≤
string
’
nameÃ"æname represents the name of this port. All ports in an EndpointSlice must have a unique name. If the EndpointSlice is derived from a Kubernetes service, this corresponds to the Service.ports[].name. Name must either be an empty string or pass DNS_LABEL validation: * must be no more than 63 characters long. * must consist of lower case alphanumeric characters or '-'. * must start and end with an alphanumeric character. Default is empty string.≤
string
ı
portÏint32"÷port represents the port number of the endpoint. If the EndpointSlice is derived from a Kubernetes service, this must be set to the service's target port. EndpointSlices used for other purposes may have a nil port.≤	
integer
v
protocolj"]protocol represents the IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.≤
string˙"
x-kubernetes-map-type	atomic

⁄
%io.k8s.api.discovery.v1.EndpointSlice∞"–EndpointSlice represents a set of service endpoints. Most EndpointSlices are created by the EndpointSlice controller to represent the Pods selected by Service objects. For a given service there may be multiple EndpointSlice objects which must be joined to produce the full set of endpoints; you can find all of the slices for a given service by listing EndpointSlices in the service's namespace whose `kubernetes.io/service-name` label contains the service's name.öaddressType≤
object ⁄
†
addressTypeê"ÇaddressType specifies the type of address carried by this EndpointSlice. All addresses in this slice must be the same type. This field is immutable after creation. The following address types are currently supported: * IPv4: Represents an IPv4 Address. * IPv6: Represents an IPv6 Address. * FQDN: Represents a Fully Qualified Domain Name. (Deprecated) The EndpointSlice controller only generates, and kube-proxy only processes, slices of addressType "IPv4" and "IPv6". No semantics are defined for the "FQDN" type.≤
string
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
ﬂ
	endpoints—"jendpoints is a list of unique endpoints in this slice. Each slice may include a maximum of 1000 endpoints.≤
array∫2
0
.#/definitions/io.k8s.api.discovery.v1.Endpoint˙#
x-kubernetes-list-type	atomic

π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
h
metadata\
=#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"Standard object's metadata.
Î
ports·"ıports specifies the list of network ports exposed by each endpoint in this slice. Each port must have a unique name. Each slice may include a maximum of 100 ports. Services always have at least 1 port, so EndpointSlices generated by the EndpointSlice controller will likewise always have at least 1 port. EndpointSlices used for other purposes may have an empty ports list.≤
array∫6
4
2#/definitions/io.k8s.api.discovery.v1.EndpointPort˙#
x-kubernetes-list-type	atomic
˙c
x-kubernetes-group-version-kind@>- group: discovery.k8s.io
  kind: EndpointSlice
  version: v1

Ω
)io.k8s.api.discovery.v1.EndpointSliceListè"6EndpointSliceList represents a list of endpoint slicesöitems≤
object ÷
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
s
itemsj"$items is the list of endpoint slices≤
array∫7
5
3#/definitions/io.k8s.api.discovery.v1.EndpointSlice
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
b
metadataV
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"Standard list metadata.˙g
x-kubernetes-group-version-kindDB- group: discovery.k8s.io
  kind: EndpointSliceList
  version: v1

√
io.k8s.api.discovery.v1.ForNodeü"LForNode provides information about which nodes should consume this endpoint.öname≤
object <
:
name2"%name represents the name of the node.≤
string
√
io.k8s.api.discovery.v1.ForZoneü"LForZone provides information about which zones should consume this endpoint.öname≤
object <
:
name2"%name represents the name of the zone.≤
string
û
-io.k8s.apimachinery.pkg.api.resource.QuantityÏ"ﬁQuantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.

The serialization format is:

``` <quantity>        ::= <signedNumber><suffix>

	(Note that <suffix> may be empty, from the "" case in <decimalSI>.)

<digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= "+" | "-" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei

	(International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)

<decimalSI>       ::= m | "" | k | M | G | T | P | E

	(Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)

<decimalExponent> ::= "e" <signedNumber> | "E" <signedNumber> ```

No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.

When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.

Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:

- No precision is lost - No fractional digits will be emitted - The exponent (or suffix) is as large as possible.

The sign will be omitted unless the number is negative.

Examples:

- 1.5 will be serialized as "1500m" - 1.5Gi will be serialized as "1536Mi"

Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.

Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)

This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.≤
string
‘
-io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup¢"YAPIGroup contains the name, the supported versions, and the preferred version of a group.önameöversions≤
object ”
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
3
name+"name is the name of the group.≤
string
«
preferredVersion≤
K#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery"cpreferredVersion is the version preferred by the API server, which probably is the storage version.
é
serverAddressByClientCIDRsÔ"Èa map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.≤
array∫P
N
L#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR˙#
x-kubernetes-list-type	atomic

√
versions∂"2versions are the versions supported in this group.≤
array∫O
M
K#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery˙#
x-kubernetes-list-type	atomic
˙P
x-kubernetes-group-version-kind-+- group: ""
  kind: APIGroup
  version: v1

ï
1io.k8s.apimachinery.pkg.apis.meta.v1.APIGroupListﬂ"RAPIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.ögroups≤
object ú
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
ú
groupsë"groups is a list of APIGroup.≤
array∫?
=
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup˙#
x-kubernetes-list-type	atomic

π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string˙T
x-kubernetes-group-version-kind1/- group: ""
  kind: APIGroupList
  version: v1

Í
0io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceµ"JAPIResource specifies the name of a resource and whether it is namespaced.önameösingularNameö
namespacedökindöverbs≤
object ®
§

categoriesï"Scategories is a list of the grouped resources this resource belongs to (e.g. 'all')≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

«
groupΩ"Øgroup is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale".≤
string
d
kind\"Okind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')≤
string
=
name5"(name is the plural name of the resource.≤
string
T

namespacedF"8namespaced indicates if a resource is namespaced or not.≤	
boolean
è

shortNamesÄ">shortNames is a list of suggested short names of the resource.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

û
singularNameç"ˇsingularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.≤
string
Ã
storageVersionHashµ"ßThe hash value of the storage version, the version this resource is converted to when written to the data store. Value must be treated as opaque by clients. Only equality comparison on the value is valid. This is an alpha feature and may change or be removed in the future. The field is populated by the apiserver only if the StorageVersionHash feature gate is enabled. This field will remain optional even if it graduates.≤
string
´
verbs°"Ñverbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)≤
array∫
≤
string
â
version˝"Ôversion is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)".≤
string
î

4io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList€	"¶APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.ögroupVersionö	resources≤
object Æ
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
_
groupVersionO"BgroupVersion is the group and version this APIResourceList is for.≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
Õ
	resourcesø"Hresources contains the name of the resources and if they are namespaced.≤
array∫B
@
>#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.APIResource˙#
x-kubernetes-list-type	atomic
˙W
x-kubernetes-group-version-kind42- group: ""
  kind: APIResourceList
  version: v1

·
0io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions¨"çAPIVersions lists the versions that are available, to allow clients to discover the API at /api, which is the root path of the legacy v1 API.översionsöserverAddressByClientCIDRs≤
object è
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
é
serverAddressByClientCIDRsÔ"Èa map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.≤
array∫P
N
L#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR˙#
x-kubernetes-list-type	atomic


versionss"1versions are the api versions that are available.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic
˙S
x-kubernetes-group-version-kind0.- group: ""
  kind: APIVersions
  version: v1

Ÿ
.io.k8s.apimachinery.pkg.apis.meta.v1.Condition¶"TCondition contains details for one aspect of the current state of this API Resource.ötypeöstatusölastTransitionTimeöreasonömessage≤
object â

∏
lastTransitionTime°
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"ÂlastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
Å
messagev"imessage is a human readable message indicating details about the transition. This may be an empty string.≤
string
»
observedGeneration±int64"õobservedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.≤	
integer
ÿ
reasonÕ"øreason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.≤
string
L
statusB"5status of the condition, one of True, False, Unknown.≤
string
T
typeL"?type of condition in CamelCase or in foo.example.com/CamelCase.≤
string
ä:
2io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions”9":DeleteOptions may be provided when deleting an API object.≤
object –
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
∆
dryRunª"¯When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

ﬁ
gracePeriodSeconds«int64"±The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.≤	
integer
ú
0ignoreStoreReadErrorWithClusterBreakingPotentialÁ"ÿif set to true, it will trigger an unsafe deletion of the resource in case the normal deletion flow fails with a corrupt object error. A resource is considered corrupt if it can not be retrieved from the underlying storage successfully because of a) its data can not be transformed e.g. decryption failure, or b) it fails to decode into an object. NOTE: unsafe deletion ignores finalizer constraints, skips precondition checks, and removes the object from the storage. WARNING: This may potentially break the cluster if the workload associated with the resource being unsafe-deleted relies on normal deletion flow. Use only if you REALLY know what you are doing. The default value is false, and the user must opt in to enable it≤	
boolean
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
ƒ
orphanDependentsØ"†Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.≤	
boolean
¬
preconditions∞
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Preconditions"lMust be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
˚
propagationPolicyÂ"◊Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.≤
string˙¥ 
x-kubernetes-group-version-kindê ç - group: ""
  kind: DeleteOptions
  version: v1
- group: admission.k8s.io
  kind: DeleteOptions
  version: v1
- group: admission.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: admissionregistration.k8s.io
  kind: DeleteOptions
  version: v1
- group: admissionregistration.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: admissionregistration.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: apiextensions.k8s.io
  kind: DeleteOptions
  version: v1
- group: apiextensions.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: apiregistration.k8s.io
  kind: DeleteOptions
  version: v1
- group: apiregistration.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: apps
  kind: DeleteOptions
  version: v1
- group: apps
  kind: DeleteOptions
  version: v1beta1
- group: apps
  kind: DeleteOptions
  version: v1beta2
- group: authentication.k8s.io
  kind: DeleteOptions
  version: v1
- group: authentication.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: authentication.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: authorization.k8s.io
  kind: DeleteOptions
  version: v1
- group: authorization.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: autoscaling
  kind: DeleteOptions
  version: v1
- group: autoscaling
  kind: DeleteOptions
  version: v2
- group: batch
  kind: DeleteOptions
  version: v1
- group: batch
  kind: DeleteOptions
  version: v1beta1
- group: certificates.k8s.io
  kind: DeleteOptions
  version: v1
- group: certificates.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: certificates.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: coordination.k8s.io
  kind: DeleteOptions
  version: v1
- group: coordination.k8s.io
  kind: DeleteOptions
  version: v1alpha2
- group: coordination.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: discovery.k8s.io
  kind: DeleteOptions
  version: v1
- group: discovery.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: events.k8s.io
  kind: DeleteOptions
  version: v1
- group: events.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: extensions
  kind: DeleteOptions
  version: v1beta1
- group: flowcontrol.apiserver.k8s.io
  kind: DeleteOptions
  version: v1
- group: flowcontrol.apiserver.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: flowcontrol.apiserver.k8s.io
  kind: DeleteOptions
  version: v1beta2
- group: flowcontrol.apiserver.k8s.io
  kind: DeleteOptions
  version: v1beta3
- group: imagepolicy.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: internal.apiserver.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: lifecycle.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: networking.k8s.io
  kind: DeleteOptions
  version: v1
- group: networking.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: node.k8s.io
  kind: DeleteOptions
  version: v1
- group: node.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: node.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: policy
  kind: DeleteOptions
  version: v1
- group: policy
  kind: DeleteOptions
  version: v1beta1
- group: rbac.authorization.k8s.io
  kind: DeleteOptions
  version: v1
- group: rbac.authorization.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: rbac.authorization.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: resource.k8s.io
  kind: DeleteOptions
  version: v1
- group: resource.k8s.io
  kind: DeleteOptions
  version: v1alpha3
- group: resource.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: resource.k8s.io
  kind: DeleteOptions
  version: v1beta2
- group: scheduling.k8s.io
  kind: DeleteOptions
  version: v1
- group: scheduling.k8s.io
  kind: DeleteOptions
  version: v1alpha3
- group: scheduling.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: storage.k8s.io
  kind: DeleteOptions
  version: v1
- group: storage.k8s.io
  kind: DeleteOptions
  version: v1alpha1
- group: storage.k8s.io
  kind: DeleteOptions
  version: v1beta1
- group: storagemigration.k8s.io
  kind: DeleteOptions
  version: v1
- group: storagemigration.k8s.io
  kind: DeleteOptions
  version: v1beta1

Ì
=io.k8s.apimachinery.pkg.apis.meta.v1.FieldSelectorRequirement´"tFieldSelectorRequirement is a selector that contains values, a key, and an operator that relates the key and values.ökeyöoperator≤
object ï
R
keyK">key is the field selector key that the requirement applies to.≤
string
∫
operator≠"üoperator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. The list of operators may grow in the future.≤
string
Å
valuesˆ"≥values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

î
-io.k8s.apimachinery.pkg.apis.meta.v1.FieldsV1‚"‘FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.

Each key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:<name>', where <name> is the name of a field in a struct, or key in a map 'v:<value>', where <value> is the exact json formatted value of a list item 'i:<index>', where <index> is position of a item in a list 'k:<keys>', where <keys> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.

The exact format is defined in sigs.k8s.io/structured-merge-diff≤
object
Ë
=io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery¶"wGroupVersion contains the "group/version" and "version" string of a version. It is made a struct to keep extensibility.ögroupVersionöversion≤
object Ö
i
groupVersionY"LgroupVersion specifies the API group and version in the form "group/version"≤
string
ó
versionã"~version specifies the version in the form of "version". This is to save the clients the trouble of splitting the GroupVersion.≤
string
€
2io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector§"ÀA label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.≤
object ¢
Ô
matchExpressions⁄"VmatchExpressions is a list of label selector requirements. The requirements are ANDed.≤
array∫O
M
K#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement˙#
x-kubernetes-list-type	atomic

≠
matchLabelsù"ˇmatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.™
≤
string≤
object˙"
x-kubernetes-map-type	atomic


=io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirementÆ"xA label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.ökeyöoperator≤
object î
F
key?"2key is the label key that the selector applies to.≤
string
é
operatorÅ"toperator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.≤
string
∏
values≠"Ívalues is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.≤
array∫
≤
string˙#
x-kubernetes-list-type	atomic

¥
-io.k8s.apimachinery.pkg.apis.meta.v1.ListMetaÇ"£ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.≤
object Õ
‰
continue◊"…continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a consistent list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response, unless you have received this token from an error message.≤
string
ﬂ
remainingItemCount»int64"≤remainingItemCount is the number of subsequent items in the list which are not included in this list response. If the list request contained label or field selectors, then the number of remaining items is unknown and the field will be left unset and omitted during serialization. If the list is complete (either because it is not chunking or because this is the last chunk), then there are no more remaining items and this field will be left unset and omitted during serialization. Servers older than v1.15 do not set this field. The intended use of the remainingItemCount is *estimating* the size of a collection. Clients should not rely on the remainingItemCount to be set or to be exact.≤	
integer
•
resourceVersionë"ÉString that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency≤
string
t
selfLinkh"[Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.≤
string
„
	shardInfo’
<#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ShardInfo"îshardInfo is set when the list is a filtered subset of the full collection, as selected by a shard selector on the request. It echoes back the selector so clients can verify which shard they received and merge sharded responses. Clients should not cache sharded list responses as a full representation of the collection.

This is an alpha field and requires enabling the ShardedListAndWatch feature gate.
ë
7io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry’"sManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.≤
object —
ñ

apiVersioná"˘APIVersion defines the version of this resource that this field set applies to. The format is "group/version" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.≤
string
†

fieldsTypeë"ÉFieldsType is the discriminator for the different fields format and version. There is currently only one possible value: "FieldsV1"≤
string
ù
fieldsV1ê
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.FieldsV1"QFieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
W
managerL"?Manager is an identifier of the workflow managing these fields.≤
string
≤
	operation§"ñOperation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.≤
string
Û
subresource„"’Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource.≤
string
Ó
timeÂ
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"©Time is the timestamp of when the ManagedFields entry was added. The timestamp will also be updated if a field is added, the manager changes any of the owned fields value or removes a field. The timestamp does not update when a field is removed from the entry because another manager took it over.
à
.io.k8s.apimachinery.pkg.apis.meta.v1.MicroTimeV	date-time">MicroTime is version of Time with microsecond level precision.≤
string
ª>
/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMetaá>"lObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.≤
object ä=
›
annotationsÕ"ØAnnotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations™
≤
string≤
object
Ó
creationTimestampÿ
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"úCreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.

Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata

deletionGracePeriodSeconds—int64"ªNumber of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.≤	
integer
§
deletionTimestampé
7#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Time"“
DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.

Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
Å

finalizersÚ"àMust be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list.≤
array∫
≤
string˙ 
x-kubernetes-list-typeset
˙'
x-kubernetes-patch-strategymerge

∆
generateNameµ"ßGenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.

If this field is specified and the generated name exists, the server will return a 409.

Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency≤
string
í

generationÉint64"nA sequence number representing a specific generation of the desired state. Populated by the system. Read-only.≤	
integer
ü
labelsî"ˆMap of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels™
≤
string≤
object
©
managedFieldsó"òManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like "ci-cd". The set of fields is always in the version that the workflow used when modifying the object.≤
array∫I
G
E#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry˙#
x-kubernetes-list-type	atomic

å
nameÉ"ıName must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names≤
string
ø
	namespace±"£Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.

Must be a DNS_LABEL. Cannot be updated. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces≤
string
ƒ
ownerReferences∞"ºList of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.≤
array∫E
C
A#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReference˙&
x-kubernetes-list-map-keys- uid
˙ 
x-kubernetes-list-typemap
˙&
x-kubernetes-patch-merge-keyuid
˙'
x-kubernetes-patch-strategymerge

ë
resourceVersion˝"ÔAn opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.

Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency≤
string
t
selfLinkh"[Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.≤
string
 
uid¬"¥UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.

Populated by the system. Read-only. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids≤
string
ó
3io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReferenceﬂ
"«OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field.ö
apiVersionökindönameöuid≤
object ¿
7

apiVersion)"API version of the referent.≤
string
â
blockOwnerDeletionÚ"„If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. See https://kubernetes.io/docs/concepts/architecture/garbage-collection/#foreground-deletion for how the garbage collector interacts with this field and enforces the foreground deletion. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.≤	
boolean
V

controllerH":If true, this reference points to the managing controller.≤	
boolean
ó
kindé"ÄKind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
É
name{"nName of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names≤
string
Ä
uidy"lUID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids≤
string˙"
x-kubernetes-map-type	atomic

ì
*io.k8s.apimachinery.pkg.apis.meta.v1.Patche"XPatch is provided to give a concrete name and type to the Kubernetes PATCH request body.≤
object
ñ
2io.k8s.apimachinery.pkg.apis.meta.v1.Preconditionsﬂ"ZPreconditions must be fulfilled before an operation (update, delete, etc.) is carried out.≤
object u
D
resourceVersion1"$Specifies the target ResourceVersion≤
string
-
uid&"Specifies the target UID.≤
string
•
>io.k8s.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR‚"çServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.ö
clientCIDRöserverAddress≤
object ¶
Ä

clientCIDRr"eThe CIDR with which clients can match their IP to figure out the server address that they should use.≤
string
†
serverAddressé"ÄAddress of this server, suitable for a client that matches the above CIDR. This can be a hostname, hostname:port, IP or IP:port.≤
string
ß
.io.k8s.apimachinery.pkg.apis.meta.v1.ShardInfoÙ"úShardInfo describes the shard selector that was applied to produce a list response. Its presence on a list response indicates the list is a filtered subset.öselector≤
object ª
∏
selector´"ùselector is the shard selector string from the request, echoed back so clients can verify which shard they received and merge responses from multiple shards.≤
string
¨
+io.k8s.apimachinery.pkg.apis.meta.v1.Status¸"CStatus is a return value for calls that don't return other objects.≤
object ◊
æ

apiVersionØ"°APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources≤
string
V
codeNint32"9Suggested HTTP return code for this status, 0 if not set.≤	
integer
¨
details†
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetails"€Extended data associated with the reason.  Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type.
π
kind∞"¢Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
U
messageJ"=A human-readable description of the status of this operation.≤
string
œ
metadata¬
;#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"ÇStandard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
‰
reasonŸ"ÀA machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.≤
string
¿
statusµ"ßStatus of the operation. One of: "Success" or "Failure". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status≤
string˙N
x-kubernetes-group-version-kind+)- group: ""
  kind: Status
  version: v1

Ü
0io.k8s.apimachinery.pkg.apis.meta.v1.StatusCause—"xStatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.≤
object »
∂
field¨"ûThe field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.

Examples:
  "name" - the field "name" on the current resource
  "items[0].name" - the field "name" on the first array entry in "items"≤
string

messaget"gA human-readable description of the cause of the error.  This field may be presented as-is to a reader.≤
string
ã
reasonÄ"sA machine-readable description of the cause of the error. If this value is empty there is no information available.≤
string
¢
2io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetailsÎ"ÈStatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.≤
object 	
Ü
causes˚"ÉThe Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.≤
array∫B
@
>#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.StatusCause˙#
x-kubernetes-list-type	atomic

b
groupY"LThe group attribute of the resource associated with the status StatusReason.≤
string
ç
kindÑ"ˆThe kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds≤
string
ó
nameé"ÄThe name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).≤
string
ó
retryAfterSecondsÅint32"ÎIf specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.≤	
integer
º
uid¥"¶UID of the resource. (when there is a single resource which can be described). More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids≤
string
Ú
)io.k8s.apimachinery.pkg.apis.meta.v1.Timeƒ	date-time"´Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.≤
string
‡"
/io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent¨""6Event represents a single event to a watched resource.ötypeöobject≤
object ◊
ø
object¥
:#/definitions/io.k8s.apimachinery.pkg.runtime.RawExtension"ıObject is:
 * If Type is Added or Modified: the new state of the object.
 * If Type is Deleted: the state of the object immediately before deletion.
 * If Type is Error: *Status is recommended; other types may make sense
   depending on context.

type≤
string˙˙
x-kubernetes-group-version-kind÷”- group: ""
  kind: WatchEvent
  version: v1
- group: admission.k8s.io
  kind: WatchEvent
  version: v1
- group: admission.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: admissionregistration.k8s.io
  kind: WatchEvent
  version: v1
- group: admissionregistration.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: admissionregistration.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: apiextensions.k8s.io
  kind: WatchEvent
  version: v1
- group: apiextensions.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: apiregistration.k8s.io
  kind: WatchEvent
  version: v1
- group: apiregistration.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: apps
  kind: WatchEvent
  version: v1
- group: apps
  kind: WatchEvent
  version: v1beta1
- group: apps
  kind: WatchEvent
  version: v1beta2
- group: authentication.k8s.io
  kind: WatchEvent
  version: v1
- group: authentication.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: authentication.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: authorization.k8s.io
  kind: WatchEvent
  version: v1
- group: authorization.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: autoscaling
  kind: WatchEvent
  version: v1
- group: autoscaling
  kind: WatchEvent
  version: v2
- group: batch
  kind: WatchEvent
  version: v1
- group: batch
  kind: WatchEvent
  version: v1beta1
- group: certificates.k8s.io
  kind: WatchEvent
  version: v1
- group: certificates.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: certificates.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: coordination.k8s.io
  kind: WatchEvent
  version: v1
- group: coordination.k8s.io
  kind: WatchEvent
  version: v1alpha2
- group: coordination.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: discovery.k8s.io
  kind: WatchEvent
  version: v1
- group: discovery.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: events.k8s.io
  kind: WatchEvent
  version: v1
- group: events.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: extensions
  kind: WatchEvent
  version: v1beta1
- group: flowcontrol.apiserver.k8s.io
  kind: WatchEvent
  version: v1
- group: flowcontrol.apiserver.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: flowcontrol.apiserver.k8s.io
  kind: WatchEvent
  version: v1beta2
- group: flowcontrol.apiserver.k8s.io
  kind: WatchEvent
  version: v1beta3
- group: imagepolicy.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: internal.apiserver.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: lifecycle.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: networking.k8s.io
  kind: WatchEvent
  version: v1
- group: networking.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: node.k8s.io
  kind: WatchEvent
  version: v1
- group: node.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: node.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: policy
  kind: WatchEvent
  version: v1
- group: policy
  kind: WatchEvent
  version: v1beta1
- group: rbac.authorization.k8s.io
  kind: WatchEvent
  version: v1
- group: rbac.authorization.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: rbac.authorization.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: resource.k8s.io
  kind: WatchEvent
  version: v1
- group: resource.k8s.io
  kind: WatchEvent
  version: v1alpha3
- group: resource.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: resource.k8s.io
  kind: WatchEvent
  version: v1beta2
- group: scheduling.k8s.io
  kind: WatchEvent
  version: v1
- group: scheduling.k8s.io
  kind: WatchEvent
  version: v1alpha3
- group: scheduling.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: storage.k8s.io
  kind: WatchEvent
  version: v1
- group: storage.k8s.io
  kind: WatchEvent
  version: v1alpha1
- group: storage.k8s.io
  kind: WatchEvent
  version: v1beta1
- group: storagemigration.k8s.io
  kind: WatchEvent
  version: v1
- group: storagemigration.k8s.io
  kind: WatchEvent
  version: v1beta1

Ö
,io.k8s.apimachinery.pkg.runtime.RawExtension‘
"∆
RawExtension is used to hold extensions in external versions.

To use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.

// Internal package:

	type MyAPIObject struct {
		runtime.TypeMeta `json:""`
		MyPlugin runtime.Object `json:"myPlugin"`
	}

	type PluginA struct {
		AOption string `json:"aOption"`
	}

// External package:

	type MyAPIObject struct {
		runtime.TypeMeta `json:""`
		MyPlugin runtime.RawExtension `json:"myPlugin"`
	}

	type PluginA struct {
		AOption string `json:"aOption"`
	}

// On the wire, the JSON will look something like this:

	{
		"kind":"MyAPIObject",
		"apiVersion":"v1",
		"myPlugin": {
			"kind":"PluginA",
			"aOption":"foo",
		},
	}

So what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)≤
object
¬
/io.k8s.apimachinery.pkg.util.intstr.IntOrStringéint-or-string"ÒIntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.≤
stringRûX
…
allowWatchBookmarks-HC2hJt-J®•¢query˜allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored."allowWatchBookmarks2boolean†
c
body-2Y1dVQaQR
Pbodybody*B
@#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions
]
body-78PwaGsrL
Jbodybody *:
8#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.Patch
Ç

continue-QfD61s0iÏ	È	Ê	query«	The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".

This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications."continue2string†
∆
fieldManager-7c6nTn1T¨©¶queryÉfieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch)."fieldManager2string†
≠
fieldManager-Qy4HdaTWìêçqueryÍfieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint."fieldManager2string†
ü
fieldSelector-xIcQKXFGÑÅquery\A selector to restrict the list of returned objects by their fields. Defaults to everything."fieldSelector2string†
ﬁ
force-tOGGb0YiÀ»≈query®Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests."force2boolean†
Å
gracePeriodSeconds--K5HaBOS·ﬁ€query±The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately."gracePeriodSeconds2integer†
‰
9ignoreStoreReadErrorWithClusterBreakingPotential-QbNkfIqj¶£†queryÿif set to true, it will trigger an unsafe deletion of the resource in case the normal deletion flow fails with a corrupt object error. A resource is considered corrupt if it can not be retrieved from the underlying storage successfully because of a) its data can not be transformed e.g. decryption failure, or b) it fails to decode into an object. NOTE: unsafe deletion ignores finalizer constraints, skips precondition checks, and removes the object from the storage. WARNING: This may potentially break the cluster if the workload associated with the resource being unsafe-deleted relies on normal deletion flow. Use only if you REALLY know what you are doing. The default value is false, and the user must opt in to enable it"0ignoreStoreReadErrorWithClusterBreakingPotential2boolean†
ü
labelSelector-5Zw57w4CÑÅquery\A selector to restrict the list of returned objects by their labels. Defaults to everything."labelSelector2string†
ä
limit-1NfNmdNH˜
Ù
Ò
query‘
limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.

The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned."limit2integer†
t
namespace-vgWSWtn3^\"Zpath:object name and auth scope, such as for teams and projects"	namespace*stringò
Ï
orphanDependents-uRB25kX5ŒÀ»query†Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both."orphanDependents2boolean†
Õ
pretty-tJGM1-ngπ∂≥queryñIf 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget)."pretty2string†
§
propagationPolicy-6jk3prlOÖÇˇquery◊Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground."propagationPolicy2string†
ï
resourceVersion-5WAnf1kx¯ıÚqueryÃresourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.

Defaults to unset"resourceVersion2string†
˘
resourceVersionMatch-t8XhRHeC◊‘—query¶resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.

Defaults to unset"resourceVersionMatch2string†
≤
sendInitialEvents-rLXlEK_kìêçquery‰
`sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic "Bookmark" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `"k8s.io/initial-events-end": "true"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.

When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan
  is interpreted as "data at least as new as the provided `resourceVersion`"
  and the bookmark event is send when the state is synced
  to a `resourceVersion` at least as fresh as the one provided by the ListOptions.
  If `resourceVersion` is unset, this is interpreted as "consistent read" and the
  bookmark event is send when the state is synced at least to the moment
  when request started being processed.
- `resourceVersionMatch` set to any other value or unset
  Invalid error is returned.

Defaults to true if `resourceVersion=""` or `resourceVersion="0"` (for backward compatibility reasons) and to false otherwise."sendInitialEvents2boolean†
Õ
shardSelector-Kgyki_3_≤Ø¨queryàshardSelector restricts the list of returned objects using a CEL-based shard selector expression. The format uses the shardRange() function combined with || (logical OR) to specify one or more hash ranges:

  shardRange(object.metadata.uid, '0x0', '0x8000000000000000')
  shardRange(object.metadata.uid, '0x0', '0x8000000000000000') || shardRange(object.metadata.uid, '0x8000000000000000', '0x10000000000000000')

Field paths use CEL-style object-rooted syntax (e.g. "object.metadata.uid"), NOT the fieldSelector format ("metadata.uid"). Currently supported paths:
  - object.metadata.uid
  - object.metadata.namespace

hexStart and hexEnd are single-quoted CEL string literals with a '0x' prefix, defining the inclusive lower and exclusive upper bounds over the 64-bit FNV-1a hash space. The full range is [0x0, 0x10000000000000000), where the exclusive upper bound equals 2^64.

Examples:
  2-shard split:
    shard 0: shardRange(object.metadata.uid, '0x0000000000000000', '0x8000000000000000')
    shard 1: shardRange(object.metadata.uid, '0x8000000000000000', '0x10000000000000000')
  4-shard split:
    shard 0: shardRange(object.metadata.uid, '0x0000000000000000', '0x4000000000000000')
    shard 1: shardRange(object.metadata.uid, '0x4000000000000000', '0x8000000000000000')
    shard 2: shardRange(object.metadata.uid, '0x8000000000000000', '0xc000000000000000')
    shard 3: shardRange(object.metadata.uid, '0xc000000000000000', '0x10000000000000000')

This is an alpha field and requires enabling the ShardedListAndWatch feature gate."shardSelector2string†
∑
timeoutSeconds-yvYezaOCõòïquerypTimeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity."timeoutSeconds2integer†
¡
watch-XNNPZGbKÆ´®queryãWatch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion."watch2boolean†b

BearerToken jO
M
BearerToken><
apiKeyauthorizationheader"Bearer Token authentication