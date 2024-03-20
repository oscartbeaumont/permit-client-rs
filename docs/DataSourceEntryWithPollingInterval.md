# DataSourceEntryWithPollingInterval

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | Url source to query for data | 
**config** | Option<[**serde_json::Value**](.md)> | Suggested fetcher configuration (e.g. auth or method) to fetch data with | [optional]
**topics** | Option<**Vec<String>**> | topics the data applies to | [optional][default to [policy_data]]
**dst_path** | Option<**String**> | OPA data api path to store the document at | [optional][default to ]
**save_method** | Option<**String**> | Method used to write into OPA - PUT/PATCH, when using the PATCH method the data field should conform to the JSON patch schema defined in RFC 6902(https://datatracker.ietf.org/doc/html/rfc6902#section-3) | [optional][default to PUT]
**data** | Option<[**models::Data**](Data.md)> |  | [optional]
**periodic_update_interval** | Option<**f64**> | Polling interval to refresh data from data source | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


