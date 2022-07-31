# \AvailabilityApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**availabilities_get**](AvailabilityApi.md#availabilities_get) | **GET** /availabilities | 
[**availabilities_id_delete**](AvailabilityApi.md#availabilities_id_delete) | **DELETE** /availabilities/{id} | 
[**availabilities_id_get**](AvailabilityApi.md#availabilities_id_get) | **GET** /availabilities/{id} | 
[**availabilities_id_put**](AvailabilityApi.md#availabilities_id_put) | **PUT** /availabilities/{id} | 
[**availabilities_post**](AvailabilityApi.md#availabilities_post) | **POST** /availabilities | 



## availabilities_get

> crate::models::AvailabilityData availabilities_get()


an availability is an amout of time where services can be booked for a worker (but may also already be booked)

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AvailabilityData**](AvailabilityData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## availabilities_id_delete

> availabilities_id_delete(id)


deletes the given availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | availability id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## availabilities_id_get

> crate::models::Availability availabilities_id_get(id)


Returns the given availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | availability id | [required] |

### Return type

[**crate::models::Availability**](Availability.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## availabilities_id_put

> crate::models::Availability availabilities_id_put(id, availability)


updates the given availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | availability id | [required] |
**availability** | [**Availability**](Availability.md) | the updated availability | [required] |

### Return type

[**crate::models::Availability**](Availability.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## availabilities_post

> crate::models::Availability availabilities_post(new_availability)


create a new availability period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_availability** | [**NewAvailability**](NewAvailability.md) | the availability to be created | [required] |

### Return type

[**crate::models::Availability**](Availability.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

