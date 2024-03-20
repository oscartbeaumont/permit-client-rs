# \PolicyDecisionPointsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_debug_audit_logs**](PolicyDecisionPointsApi.md#disable_debug_audit_logs) | **PUT** /v2/pdps/{proj_id}/{env_id}/configs/{pdp_id}/debug-audit-logs/disable | Disable debug audit logs
[**enable_debug_audit_logs**](PolicyDecisionPointsApi.md#enable_debug_audit_logs) | **PUT** /v2/pdps/{proj_id}/{env_id}/configs/{pdp_id}/debug-audit-logs/enable | Enable debug audit logs
[**list_pdp_configs**](PolicyDecisionPointsApi.md#list_pdp_configs) | **GET** /v2/pdps/{proj_id}/{env_id}/configs | List PDP configurations
[**migrate_shards**](PolicyDecisionPointsApi.md#migrate_shards) | **POST** /v2/pdps/{proj_id}/{env_id}/configs/migrate-shards | Migrate PDP Config number of shards
[**rotate_pdp_api_key**](PolicyDecisionPointsApi.md#rotate_pdp_api_key) | **POST** /v2/pdps/{proj_id}/{env_id}/configs/{pdp_id}/rotate-api-key | Rotate PDP API Key
[**update_min_pdp_version**](PolicyDecisionPointsApi.md#update_min_pdp_version) | **PATCH** /v2/pdps/{proj_id}/{env_id}/configs/{pdp_id}/min-pdp-version | Update minimum PDP version



## disable_debug_audit_logs

> models::PdpConfigRead disable_debug_audit_logs(proj_id, env_id, pdp_id)
Disable debug audit logs

Disabled debug audit logs for the PDP container with id `pdp_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**pdp_id** | **uuid::Uuid** | The unique id of the pdp | [required] |

### Return type

[**models::PdpConfigRead**](PDPConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_debug_audit_logs

> models::PdpConfigRead enable_debug_audit_logs(proj_id, env_id, pdp_id)
Enable debug audit logs

Enables debug audit logs for the PDP container with id `pdp_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**pdp_id** | **uuid::Uuid** | The unique id of the pdp | [required] |

### Return type

[**models::PdpConfigRead**](PDPConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pdp_configs

> Vec<models::PdpConfigRead> list_pdp_configs(proj_id, env_id, page, per_page)
List PDP configurations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::PdpConfigRead>**](PDPConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_shards

> models::PdpConfigRead migrate_shards(proj_id, env_id, pdp_shard_migration)
Migrate PDP Config number of shards

The migration process is as followed: 1. Perform request to this endpoint with the new number of shards 2. A new PDP Config will be created with the new number of shards and a new api-key 3. Create a new PDP cluster with the same instances as the number of shards defined in the new PDP Config 4. Wait for the new PDP cluster to be ready 5. Update your PDP load balancer to point to the new PDP cluster  More info can be found here https://docs.permit.io/concepts/pdp-sharding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**pdp_shard_migration** | [**PdpShardMigration**](PdpShardMigration.md) |  | [required] |

### Return type

[**models::PdpConfigRead**](PDPConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_pdp_api_key

> models::PdpConfigRead rotate_pdp_api_key(proj_id, env_id, pdp_id)
Rotate PDP API Key

Rotates the API key of the PDP container with id `pdp_id`.  The rotation of the API key revokes the old API key and issues a new API key to the PDP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**pdp_id** | **uuid::Uuid** | The unique id of the pdp | [required] |

### Return type

[**models::PdpConfigRead**](PDPConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_min_pdp_version

> models::PdpConfigRead update_min_pdp_version(proj_id, env_id, pdp_id, pdp_config_version)
Update minimum PDP version

Update the minimum PDP version for the PDP container with id `pdp_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**pdp_id** | **uuid::Uuid** | The unique id of the pdp | [required] |
**pdp_config_version** | [**PdpConfigVersion**](PdpConfigVersion.md) |  | [required] |

### Return type

[**models::PdpConfigRead**](PDPConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

