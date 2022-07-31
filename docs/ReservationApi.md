# \ReservationApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reservations_get**](ReservationApi.md#reservations_get) | **GET** /reservations | 
[**reservations_id_delete**](ReservationApi.md#reservations_id_delete) | **DELETE** /reservations/{id} | 
[**reservations_id_get**](ReservationApi.md#reservations_id_get) | **GET** /reservations/{id} | 
[**reservations_id_put**](ReservationApi.md#reservations_id_put) | **PUT** /reservations/{id} | 
[**reservations_id_tags_put**](ReservationApi.md#reservations_id_tags_put) | **PUT** /reservations/{id}/tags | 
[**reservations_post**](ReservationApi.md#reservations_post) | **POST** /reservations | 



## reservations_get

> crate::models::ReservationData reservations_get(start_time_min, start_time_max, tag_id)


a reservation is an amout of time booked by a patient or blocked by the tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time_min** | Option<**String**> | only return results with a startTime of at least this value (inclusive) |  |
**start_time_max** | Option<**String**> | only return results with a startTime of less than this value (exclusive) |  |
**tag_id** | Option<**String**> | only return results with this tag ID set |  |

### Return type

[**crate::models::ReservationData**](ReservationData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reservations_id_delete

> reservations_id_delete(id)


deletes the given reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | reservation id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reservations_id_get

> crate::models::Reservation reservations_id_get(id)


Returns the given reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | reservation id | [required] |

### Return type

[**crate::models::Reservation**](Reservation.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reservations_id_put

> crate::models::Reservation reservations_id_put(id, reservation)


updates the given reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | reservation id | [required] |
**reservation** | [**Reservation**](Reservation.md) | the updated reservation | [required] |

### Return type

[**crate::models::Reservation**](Reservation.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reservations_id_tags_put

> crate::models::Reservation reservations_id_tags_put(id, reservation_tags)


updates (only) the tags of a given reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | reservation id | [required] |
**reservation_tags** | [**ReservationTags**](ReservationTags.md) | the updated list of tags | [required] |

### Return type

[**crate::models::Reservation**](Reservation.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reservations_post

> crate::models::Reservation reservations_post(new_reservation)


create a new reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_reservation** | [**NewReservation**](NewReservation.md) | the reservation to be created | [required] |

### Return type

[**crate::models::Reservation**](Reservation.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

