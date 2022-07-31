# \WidgetApiApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**booking_config_tid_get**](WidgetApiApi.md#booking_config_tid_get) | **GET** /booking/config/{tid} | 
[**booking_reserve_post**](WidgetApiApi.md#booking_reserve_post) | **POST** /booking/reserve | 
[**booking_reserve_rid_confirmation_post**](WidgetApiApi.md#booking_reserve_rid_confirmation_post) | **POST** /booking/reserve/{rid}/confirmation | 
[**booking_reserve_rid_put**](WidgetApiApi.md#booking_reserve_rid_put) | **PUT** /booking/reserve/{rid} | 
[**booking_schedule_tid_sid_wid_date_get**](WidgetApiApi.md#booking_schedule_tid_sid_wid_date_get) | **GET** /booking/schedule/{tid}/{sid}/{wid}/{date} | 



## booking_config_tid_get

> crate::models::TenantConfig booking_config_tid_get(tid)


# Provides basic configuration information for the widget of the specified tenant.  It is the entry point for the widget. The widget only needs to know in the page of which tenant it is used, then it sends out a request to this endpoint to know its basic configuration, services and workers it can offer to the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** | ID in UUID format of the tenant to get the configuration for  | [required] |

### Return type

[**crate::models::TenantConfig**](TenantConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_reserve_post

> crate::models::ReservationResponse booking_reserve_post(reservation_request)


# Reserve a time slot for a user  After the user has selected a time slot, that it wants to use, the widget will use this endpoint to reserve the given time slot. The server will check, whether the slot is still available, send out the verification code to the user by SMS or phone call, and return to the caller whether the slot could be reserved.  For final booking of the time slot, the user has to give the widget the PIN it got sent, and the widget has to send the PIN with another request to the server, to finally book the reservation it made. Reservations, that do not get booked, will be canceled after a configured amount of time.  **Note:** This endpoint is especially purposed for the self-service of the user using the booking widget, therefore a reservation created here will be stored with a source type of `WEB` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_request** | [**ReservationRequest**](ReservationRequest.md) | information about the requested time slot to be reserved | [required] |

### Return type

[**crate::models::ReservationResponse**](ReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_reserve_rid_confirmation_post

> crate::models::ReservationConfirmationResponse booking_reserve_rid_confirmation_post(rid, reservation_confirmation_request)


# Confirm a reservation with the PIN sent to the user  After the user has entered the PIN he received in the Widget, the widget has to send the PIN to the server using this function. This on the one hand will confirm the reservation to the server, and on the other hand will tell the widget whether the entered PIN has been correct. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rid** | **String** | ID of the reservation that gets confirmed | [required] |
**reservation_confirmation_request** | [**ReservationConfirmationRequest**](ReservationConfirmationRequest.md) | information about the requested time slot to be reserved | [required] |

### Return type

[**crate::models::ReservationConfirmationResponse**](ReservationConfirmationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_reserve_rid_put

> crate::models::ReservationResponse booking_reserve_rid_put(rid, reservation_request)


# Update a reservation  This has two purposes: - it can be used to change the phone number or other data used when a reservation has been created (e.g. because   a user detected, that he made a typo) - after updating (even with the same data as before) the verification PIN is resent, so it can be used if the   user did not receive the PIN and the PIN has to be sent another time  Note: updates to a reservation are only possible as long as the reservation has not been confirmed by the user. Once the user confirms the reservation by entering the PIN he has received from the system, the reservation is fixed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rid** | **String** | ID of the reservation that gets updated | [required] |
**reservation_request** | [**ReservationRequest**](ReservationRequest.md) | updated data for the reservation (can be the same as the original reservation, if only a resend of the PIN is what is needed) | [required] |

### Return type

[**crate::models::ReservationResponse**](ReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_schedule_tid_sid_wid_date_get

> crate::models::Availabilities booking_schedule_tid_sid_wid_date_get(tid, sid, wid, date)


# Returns the available times at which a service can be booked.  After the user has selected a service and a worker, the Widget will use this endpoint to request the time slots that it can provide to the user. Time slots are requested by date. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** | ID in UUID format of the tenant to get the timestamps for  | [required] |
**sid** | **String** | ID in UUID format of the service to get timestamps for  | [required] |
**wid** | **String** | ID in UUID format of the worker to get timestamps for  | [required] |
**date** | **String** | The date to get times for, if there are no available time slots for this date, the server will return the first day after this date, where time slots are available (if any). If the server returns no time slots, than there are no available slots given on that date or any later date.  | [required] |

### Return type

[**crate::models::Availabilities**](Availabilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

