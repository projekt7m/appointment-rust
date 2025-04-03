# Availability

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**worker_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**day_of_week** | [**models::DayOfWeek**](DayOfWeek.md) |  | 
**start_time** | **String** |  | 
**end_time** | **String** |  | 
**services** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | 
**not_before** | Option<[**String**](string.md)> |  | [optional]
**not_after** | Option<[**String**](string.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


