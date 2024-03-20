# RelationshipTupleCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | **String** | the resource instance assigned the new relation (accepts either the resource instance id or resource_key:resource_instance_key) | 
**relation** | **String** | the relation to assign between the subject and object | 
**object** | **String** | the resource instance on which the new relation is assigned (accepts either the resource instance id or resource_key:resource_instance_key) | 
**tenant** | Option<**String**> | The tenant the subject and object belong to, if the resource instances don't exist yet, the tenant is required to create them. otherwise it is ignored | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


