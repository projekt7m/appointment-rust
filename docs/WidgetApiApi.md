# \WidgetApiApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_booking_config**](WidgetApiApi.md#get_booking_config) | **GET** /booking/config/{id} | Get the basic configuration information for the widget of the specified tenant
[**get_booking_schedule**](WidgetApiApi.md#get_booking_schedule) | **GET** /booking/schedule/{tid}/{sid}/{wid}/{date} | Get the available times at which a service can be booked.
[**post_booking_reserve**](WidgetApiApi.md#post_booking_reserve) | **POST** /booking/reserve | Reserve a time slot for a user
[**post_booking_reserve_confirmation**](WidgetApiApi.md#post_booking_reserve_confirmation) | **POST** /booking/reserve/{rid}/confirmation | Confirm a reservation with the PIN sent to the user
[**put_booking_reserve_by_id**](WidgetApiApi.md#put_booking_reserve_by_id) | **PUT** /booking/reserve/{rid} | Resend the code for an existing reservation



## get_booking_config

> crate::models::TenantConfig get_booking_config(id)
Get the basic configuration information for the widget of the specified tenant

Get the basic configuration information for the widget of the specified tenant  It is the entry point for the widget. The widget only needs to know in the page of which tenant it is used, then it sends out a request to this endpoint to know its basic configuration, services and workers it can offer to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID in UUID format of the tenant to get the configuration for | [required] |

### Return type

[**crate::models::TenantConfig**](TenantConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_booking_schedule

> crate::models::Availabilities get_booking_schedule(tid, sid, wid, date)
Get the available times at which a service can be booked.

Get the available times at which a service can be booked.  After the user has selected a service and a worker, the Widget will use this endpoint to request the time slots it can provide to the user. Time slots are requested by date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** | ID of the tenant to get the timestamps for | [required] |
**sid** | **String** | ID of the service to get timestamps for | [required] |
**wid** | **String** | ID of the worker to get timestamps for | [required] |
**date** | **String** | The date to get times for, if there are no available time slots for this date, the server will return the first day after this date, where time slots are available (if any). If the server returns no time slots, than there are no available slots given on that date or any later date. | [required] |

### Return type

[**crate::models::Availabilities**](Availabilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_booking_reserve

> crate::models::ReservationResponse post_booking_reserve(reservation_request)
Reserve a time slot for a user

Reserve a time slot for a user  After the user has selected a time slot, that it wants to use, the widget will use this endpoint to reserve the given time slot. The server will check, whether the slot is still available, send out the verification code to the user by SMS or phone call, and return to the caller whether the slot could be reserved.  For final booking of the time slot, the user has to give the widget the PIN it got sent, and the widget has to send the PIN with another request to the server, to finally book the reservation it made. Reservations, that do not get booked, will be canceled after a configured amount of time.  **Note:** This endpoint is especially purposed for the self-service of the user using the booking widget, therefore a reservation created here will be stored with a source type of `WEB`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_request** | [**ReservationRequest**](ReservationRequest.md) | Information about the requested time slot to be reserved. | [required] |

### Return type

[**crate::models::ReservationResponse**](ReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_booking_reserve_confirmation

> crate::models::ConfirmationResponse post_booking_reserve_confirmation(rid, confirmation_request)
Confirm a reservation with the PIN sent to the user

Confirm a reservation with the PIN sent to the user  After the user has entered the PIN he received in the Widget, the widget has to send the PIN to the server using this function. On the one hand this will confirm the reservation to the server, and on the other hand will tell the widget whether the entered PIN has been correct.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rid** | **String** | ID of the reservation that gets confirmed | [required] |
**confirmation_request** | [**ConfirmationRequest**](ConfirmationRequest.md) | The PIN the user entered | [required] |

### Return type

[**crate::models::ConfirmationResponse**](ConfirmationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_booking_reserve_by_id

> crate::models::ReservationResponse put_booking_reserve_by_id(rid, reservation_request)
Resend the code for an existing reservation

Resend the code for an existing reservation  TODO: Use this endpoint also to allow updating a revervation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rid** | **String** | ID of the reservation that gets updated | [required] |
**reservation_request** | [**ReservationRequest**](ReservationRequest.md) | The reservation for which the code should get resent | [required] |

### Return type

[**crate::models::ReservationResponse**](ReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

