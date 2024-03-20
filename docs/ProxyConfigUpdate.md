# ProxyConfigUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**secret** | Option<[**models::Secret**](Secret.md)> |  | [optional]
**name** | Option<**String**> | The name of the proxy config, for example: 'Stripe API' | [optional]
**mapping_rules** | Option<[**Vec<models::MappingRule>**](MappingRule.md)> | Proxy config mapping rules will include the rules that will be used to map the request to the backend service by a URL and a http method. | [optional][default to []]
**auth_mechanism** | Option<[**models::AuthMechanism**](AuthMechanism.md)> | Proxy config auth mechanism will define the authentication mechanism that will be used to authenticate the request.  Bearer injects the secret into the Authorization header as a Bearer token,  Basic injects the secret into the Authorization header as a Basic user:password,  Headers injects plain headers into the request. | [optional][default to Bearer]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


