# \ProjectsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectsApi.md#create_project) | **POST** /v2/projects | Create Project
[**delete_project**](ProjectsApi.md#delete_project) | **DELETE** /v2/projects/{proj_id} | Delete Project
[**get_project**](ProjectsApi.md#get_project) | **GET** /v2/projects/{proj_id} | Get Project
[**list_projects**](ProjectsApi.md#list_projects) | **GET** /v2/projects | List Projects
[**update_project**](ProjectsApi.md#update_project) | **PATCH** /v2/projects/{proj_id} | Update Project



## create_project

> models::ProjectRead create_project(project_create)
Create Project

Creates a new project under the active organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_create** | [**ProjectCreate**](ProjectCreate.md) |  | [required] |

### Return type

[**models::ProjectRead**](ProjectRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> delete_project(proj_id)
Delete Project

Deletes the project and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> models::ProjectRead get_project(proj_id)
Get Project

Gets a single project matching the given proj_id, if such project exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |

### Return type

[**models::ProjectRead**](ProjectRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> Vec<models::ProjectRead> list_projects(page, per_page)
List Projects

Lists all the projects under the active organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ProjectRead>**](ProjectRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> models::ProjectRead update_project(proj_id, project_update)
Update Project

Updates the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**project_update** | [**ProjectUpdate**](ProjectUpdate.md) |  | [required] |

### Return type

[**models::ProjectRead**](ProjectRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

