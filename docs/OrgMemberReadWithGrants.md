# OrgMemberReadWithGrants

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the account member | 
**email** | **String** | Email of the user controlling this account | 
**email_verified** | **bool** | Whether this email address is verified or not. For social providers like 'Login with Google' this is done automatically, otherwise we will send the user a verification link in email. | 
**name** | Option<**String**> | Name of this user | [optional]
**given_name** | Option<**String**> | First name of the user | [optional]
**family_name** | Option<**String**> | Last name of the user | [optional]
**picture** | Option<**String**> | URL to picture, photo, or avatar of the user that controls this account. | [optional]
**is_superuser** | **bool** | Whether or not this user has special access to permit.io organizations | 
**is_onboarding** | **bool** | Whether or not this user is currently onboarding, needs to be replaced by a user journey object | 
**onboarding_step** | [**models::OnboardingStep**](OnboardingStep.md) | the step the user is currently going through in onboarding | 
**created_at** | **String** | Date and time when the account member was created (ISO_8601 format). | 
**last_login** | Option<**String**> | Last date and time this user logged in (ISO_8601 format). | [optional]
**last_ip** | Option<**String**> | Last IP address from which this user logged in. | [optional][default to 0.0.0.0]
**logins_count** | Option<**i32**> | Total number of logins this user has performed. | [optional][default to 0]
**identities** | [**Vec<models::IdentityRead>**](IdentityRead.md) |  | 
**invite** | Option<[**models::InviteRead**](InviteRead.md)> |  | [optional]
**settings** | [**serde_json::Value**](.md) | Custom permit.io dashboard settings, such as preferred theme, etc. | 
**grants** | [**Vec<models::Permission>**](Permission.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


