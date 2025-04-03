# ListWrapperReservationDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reservation_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**worker_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**service_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | [**models::ReservationType**](ReservationType.md) |  | 
**start_time** | **String** |  | 
**end_time** | **String** |  | 
**description** | **String** |  | 
**patient_name** | Option<**String**> |  | [optional]
**patient_phone** | Option<**String**> |  | [optional]
**patient_mail** | Option<**String**> |  | [optional]
**patient_birthday** | Option<[**String**](string.md)> |  | [optional]
**reserved_at** | **String** |  | 
**confirmed_at** | Option<**String**> |  | [optional]
**confirmation_pin** | Option<**String**> |  | [optional]
**confirmation_sent** | **bool** |  | 
**reminder_sent** | **bool** |  | 
**client_notified** | **bool** |  | 
**tags** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | 
**answers** | **String** |  | 
**answer_shorthands** | **String** |  | 
**source** | [**models::SourceMedium**](SourceMedium.md) |  | 
**notes** | Option<**String**> |  | [optional]
**send_sms_messages** | Option<**bool**> |  | [optional]
**send_mail_messages** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


