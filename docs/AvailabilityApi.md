# \AvailabilityApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_availability_by_id**](AvailabilityApi.md#delete_availability_by_id) | **DELETE** /availabilities/{id} | Delete an availability
[**get_availabilities**](AvailabilityApi.md#get_availabilities) | **GET** /availabilities | Get the list of availabilities
[**get_availability_by_id**](AvailabilityApi.md#get_availability_by_id) | **GET** /availabilities/{id} | Request a single availability
[**post_availabilities**](AvailabilityApi.md#post_availabilities) | **POST** /availabilities | Create a new availability period
[**put_availability_by_id**](AvailabilityApi.md#put_availability_by_id) | **PUT** /availabilities/{id} | Update a single availability



## delete_availability_by_id

> delete_availability_by_id(id)
Delete an availability

Delete an availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the availability | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_availabilities

> crate::models::AvailabilityData get_availabilities()
Get the list of availabilities

Get the list of availabilities  An availability is an amout of time where services can be booked for a worker (But may also already be booked)

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AvailabilityData**](AvailabilityData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_availability_by_id

> crate::models::Availability get_availability_by_id(id)
Request a single availability

Request a single availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the availability | [required] |

### Return type

[**crate::models::Availability**](Availability.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_availabilities

> crate::models::Availability post_availabilities(new_availability)
Create a new availability period

Create a new availability period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_availability** | [**NewAvailability**](NewAvailability.md) | The availability to be created | [required] |

### Return type

[**crate::models::Availability**](Availability.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_availability_by_id

> crate::models::Availability put_availability_by_id(id, new_availability)
Update a single availability

Update a single availability

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the availability | [required] |
**new_availability** | [**NewAvailability**](NewAvailability.md) | The updated availability | [required] |

### Return type

[**crate::models::Availability**](Availability.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

