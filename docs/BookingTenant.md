# BookingTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**timezone** | **String** |  | 
**reservation_limit_mode** | [**models::ReservationLimit**](ReservationLimit.md) |  | 
**reservation_limit_date** | Option<[**String**](string.md)> |  | [optional]
**reservation_limit_days** | Option<**i32**> |  | [optional]
**address** | **String** |  | 
**phone** | **String** |  | 
**fax** | **String** |  | 
**email** | **String** |  | 
**email_signature** | **String** |  | 
**name** | **String** |  | 
**email_internal** | **String** |  | 
**calendar_extra_info** | **String** |  | 
**required_conditions** | **Vec<String>** |  | 
**holiday_region_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


