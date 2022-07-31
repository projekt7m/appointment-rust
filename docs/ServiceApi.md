# \ServiceApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**services_get**](ServiceApi.md#services_get) | **GET** /services | 
[**services_id_delete**](ServiceApi.md#services_id_delete) | **DELETE** /services/{id} | 
[**services_id_get**](ServiceApi.md#services_id_get) | **GET** /services/{id} | 
[**services_id_put**](ServiceApi.md#services_id_put) | **PUT** /services/{id} | 
[**services_post**](ServiceApi.md#services_post) | **POST** /services | 



## services_get

> crate::models::ServiceData services_get()


a service is an action that can be selected and booked by a patient

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServiceData**](ServiceData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## services_id_delete

> services_id_delete(id)


deletes the given service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | service id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## services_id_get

> crate::models::Service services_id_get(id)


Returns the given service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | service id | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## services_id_put

> crate::models::Service services_id_put(id, service)


updates the given service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | service id | [required] |
**service** | [**Service**](Service.md) | the updated service | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## services_post

> crate::models::Service services_post(new_service)


create a new service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_service** | [**NewService**](NewService.md) | the service to be created | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

