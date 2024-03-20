# \OrganizationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization**](OrganizationsApi.md#create_organization) | **POST** /v2/orgs | Create Organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /v2/orgs/{org_id} | Delete Organization
[**get_active_organization**](OrganizationsApi.md#get_active_organization) | **GET** /v2/orgs/active/org | Get Active Organization
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /v2/orgs/{org_id} | Get Organization
[**list_organizations**](OrganizationsApi.md#list_organizations) | **GET** /v2/orgs | List Organizations
[**stats_organization**](OrganizationsApi.md#stats_organization) | **GET** /v2/orgs/{org_id}/stats | Stats Organization
[**update_organization**](OrganizationsApi.md#update_organization) | **PATCH** /v2/orgs/{org_id} | Update Organization



## create_organization

> models::OrganizationReadWithApiKey create_organization(organization_create)
Create Organization

Creates a new organization that will be owned by the authenticated actor (i.e: human team member or api key).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_create** | [**OrganizationCreate**](OrganizationCreate.md) |  | [required] |

### Return type

[**models::OrganizationReadWithApiKey**](OrganizationReadWithAPIKey.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(org_id)
Delete Organization

Deletes an organization (Permit.io account) and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | Either the unique id of the organization, or the URL-friendly key of the organization (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_organization

> models::OrganizationRead get_active_organization()
Get Active Organization

Gets a single organization (Permit.io account) matching the given org_id, if such org exists and can be accessed by the authenticated actor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrganizationRead**](OrganizationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::OrganizationRead get_organization(org_id)
Get Organization

Gets a single organization (Permit.io account) matching the given org_id, if such org exists and can be accessed by the authenticated actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | Either the unique id of the organization, or the URL-friendly key of the organization (i.e: the \"slug\"). | [required] |

### Return type

[**models::OrganizationRead**](OrganizationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organizations

> Vec<models::OrganizationRead> list_organizations(page, per_page)
List Organizations

Lists all the organizations that can be accessed by the authenticated actor (i.e: human team member or api key).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::OrganizationRead>**](OrganizationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats_organization

> models::OrganizationStats stats_organization(org_id)
Stats Organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | Either the unique id of the organization, or the URL-friendly key of the organization (i.e: the \"slug\"). | [required] |

### Return type

[**models::OrganizationStats**](OrganizationStats.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> models::OrganizationRead update_organization(org_id, organization_update)
Update Organization

Updates the organization's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | Either the unique id of the organization, or the URL-friendly key of the organization (i.e: the \"slug\"). | [required] |
**organization_update** | [**OrganizationUpdate**](OrganizationUpdate.md) |  | [required] |

### Return type

[**models::OrganizationRead**](OrganizationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

