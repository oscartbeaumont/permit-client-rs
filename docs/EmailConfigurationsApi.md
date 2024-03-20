# \EmailConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_email_configuration**](EmailConfigurationsApi.md#create_or_update_email_configuration) | **POST** /v2/facts/{proj_id}/{env_id}/email_configurations | Create Or Update Email Configuration
[**get_email_configuration**](EmailConfigurationsApi.md#get_email_configuration) | **GET** /v2/facts/{proj_id}/{env_id}/email_configurations | Get Email Configuration
[**send_test_email**](EmailConfigurationsApi.md#send_test_email) | **POST** /v2/facts/{proj_id}/{env_id}/email_configurations/send_test_email | Send Test Email



## create_or_update_email_configuration

> models::EmailConfigurationRead create_or_update_email_configuration(proj_id, env_id, email_configuration_create)
Create Or Update Email Configuration

Create new configuration or updates the email configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**email_configuration_create** | [**EmailConfigurationCreate**](EmailConfigurationCreate.md) |  | [required] |

### Return type

[**models::EmailConfigurationRead**](EmailConfigurationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_configuration

> models::EmailConfigurationRead get_email_configuration(proj_id, env_id)
Get Email Configuration

Gets the email configuration defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |

### Return type

[**models::EmailConfigurationRead**](EmailConfigurationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_email

> serde_json::Value send_test_email(proj_id, env_id, email_configuration_create)
Send Test Email

Sends a test email to the email address defined in the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**email_configuration_create** | [**EmailConfigurationCreate**](EmailConfigurationCreate.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

