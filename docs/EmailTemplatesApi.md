# \EmailTemplatesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_template_by_type**](EmailTemplatesApi.md#get_template_by_type) | **GET** /v2/facts/{proj_id}/{env_id}/email_templates/{template_type} | Get Template By Type
[**list_templates**](EmailTemplatesApi.md#list_templates) | **GET** /v2/facts/{proj_id}/{env_id}/email_templates/ | List Templates
[**send_test_email_by_type**](EmailTemplatesApi.md#send_test_email_by_type) | **POST** /v2/facts/{proj_id}/{env_id}/email_templates/{template_type}/send_test_email | Send Test Email By Type
[**update_template_by_type**](EmailTemplatesApi.md#update_template_by_type) | **POST** /v2/facts/{proj_id}/{env_id}/email_templates/{template_type} | Update Template By Type



## get_template_by_type

> models::EmailTemplateRead get_template_by_type(proj_id, env_id, template_type)
Get Template By Type

Lists all the email configurations defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**template_type** | [**EmailTemplateType**](.md) |  | [required] |

### Return type

[**models::EmailTemplateRead**](EmailTemplateRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates

> Vec<models::EmailTemplateRead> list_templates(proj_id, env_id, page, per_page)
List Templates

Lists all the email configurations defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::EmailTemplateRead>**](EmailTemplateRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_email_by_type

> serde_json::Value send_test_email_by_type(template_type, proj_id, env_id, email_template_update)
Send Test Email By Type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | [**EmailTemplateType**](.md) |  | [required] |
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**email_template_update** | [**EmailTemplateUpdate**](EmailTemplateUpdate.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_template_by_type

> models::EmailTemplateRead update_template_by_type(proj_id, env_id, template_type, email_template_update)
Update Template By Type

Updates an email template by a given type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**template_type** | [**EmailTemplateType**](.md) |  | [required] |
**email_template_update** | [**EmailTemplateUpdate**](EmailTemplateUpdate.md) |  | [required] |

### Return type

[**models::EmailTemplateRead**](EmailTemplateRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

