# \ReservationApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_reservation_by_id**](ReservationApi.md#delete_reservation_by_id) | **DELETE** /reservations/{id} | Delete the reservation with the given ID
[**get_reservations**](ReservationApi.md#get_reservations) | **GET** /reservations | Get all reservations
[**get_reservations_by_id**](ReservationApi.md#get_reservations_by_id) | **GET** /reservations/{id} | Get a single reservation identified by its ID
[**post_reservations**](ReservationApi.md#post_reservations) | **POST** /reservations | Create a new reservation
[**put_reservation_by_id**](ReservationApi.md#put_reservation_by_id) | **PUT** /reservations/{id} | Update an existing reservation
[**put_reservation_tags_by_id**](ReservationApi.md#put_reservation_tags_by_id) | **PUT** /reservations/{id}/tags | Update (only) the tags of a given reservation



## delete_reservation_by_id

> delete_reservation_by_id(id)
Delete the reservation with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the reservation | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reservations

> models::ListWrapperReservation get_reservations(start_time_min, start_time_max, tag_id)
Get all reservations

A reservation is an amount of time booked by a patient or blocked by the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time_min** | Option<**String**> | only return results with a startTime of at least this value (inclusive) |  |
**start_time_max** | Option<**String**> | only return results with a startTime of less than this value (exclusive) |  |
**tag_id** | Option<**uuid::Uuid**> | only return results with this tag ID set |  |

### Return type

[**models::ListWrapperReservation**](ListWrapper_Reservation.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reservations_by_id

> models::Reservation get_reservations_by_id(id)
Get a single reservation identified by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the reservation | [required] |

### Return type

[**models::Reservation**](Reservation.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_reservations

> models::Reservation post_reservations(new_reservation)
Create a new reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_reservation** | [**NewReservation**](NewReservation.md) | The reservation to be created | [required] |

### Return type

[**models::Reservation**](Reservation.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_reservation_by_id

> models::Reservation put_reservation_by_id(id, reservation)
Update an existing reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the reservation | [required] |
**reservation** | [**Reservation**](Reservation.md) | The updated reservation | [required] |

### Return type

[**models::Reservation**](Reservation.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_reservation_tags_by_id

> models::Reservation put_reservation_tags_by_id(id, uuid_colon_colon_uuid)
Update (only) the tags of a given reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the reservation | [required] |
**uuid_colon_colon_uuid** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | The updated list of tags | [required] |

### Return type

[**models::Reservation**](Reservation.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

