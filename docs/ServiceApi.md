# \ServiceApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_service_by_id**](ServiceApi.md#delete_service_by_id) | **DELETE** /services/{id} | Delete a service by its ID
[**get_service_by_id**](ServiceApi.md#get_service_by_id) | **GET** /services/{id} | Get a service by its ID
[**get_services**](ServiceApi.md#get_services) | **GET** /services | Get list of all services
[**post_service**](ServiceApi.md#post_service) | **POST** /services | Create a new service
[**put_service_by_id**](ServiceApi.md#put_service_by_id) | **PUT** /services/{id} | Updates a single service



## delete_service_by_id

> delete_service_by_id(id)
Delete a service by its ID

Delete a service by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the service | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_by_id

> crate::models::Service get_service_by_id(id)
Get a service by its ID

Get a service by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the service | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> crate::models::ServiceData get_services()
Get list of all services

Get list of all services  A service is an action that can be selected and booked by a patient

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServiceData**](ServiceData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_service

> crate::models::Service post_service(new_service)
Create a new service

Create a new service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_service** | [**NewService**](NewService.md) | The service to be created | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_service_by_id

> crate::models::Service put_service_by_id(id, new_service)
Updates a single service

Updates a single service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the service | [required] |
**new_service** | [**NewService**](NewService.md) | The updated service | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

