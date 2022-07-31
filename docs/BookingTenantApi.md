# \BookingTenantApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**booking_tenant_get**](BookingTenantApi.md#booking_tenant_get) | **GET** /booking/tenant | 
[**booking_tenant_id_delete**](BookingTenantApi.md#booking_tenant_id_delete) | **DELETE** /booking/tenant/{id} | 
[**booking_tenant_id_get**](BookingTenantApi.md#booking_tenant_id_get) | **GET** /booking/tenant/{id} | 
[**booking_tenant_id_put**](BookingTenantApi.md#booking_tenant_id_put) | **PUT** /booking/tenant/{id} | 
[**booking_tenant_post**](BookingTenantApi.md#booking_tenant_post) | **POST** /booking/tenant | 



## booking_tenant_get

> crate::models::BookingTenantData booking_tenant_get()


a booking tenant is a tenant as the appointment API sees it  Data here should more or less mirror the information of the Tenant as it is configured in the authentication and user API. Especially the `tenantId` must match the value used there, else the services can not relate data to the data of a different micro service. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BookingTenantData**](BookingTenantData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_tenant_id_delete

> booking_tenant_id_delete(id)


deletes the specified booking tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | (booking) tenant id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_tenant_id_get

> crate::models::BookingTenant booking_tenant_id_get(id)


Returns the specified booking tenant 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | (booking) tenant id | [required] |

### Return type

[**crate::models::BookingTenant**](BookingTenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_tenant_id_put

> crate::models::BookingTenant booking_tenant_id_put(id, booking_tenant)


updates the specified booking tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | (booking) tenant id | [required] |
**booking_tenant** | [**BookingTenant**](BookingTenant.md) | the updated booking tenant | [required] |

### Return type

[**crate::models::BookingTenant**](BookingTenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## booking_tenant_post

> crate::models::BookingTenant booking_tenant_post(booking_tenant)


a booking tenant is a tenant as the appointment API sees it  Data here should more or less mirror the information of the Tenant as it is configured in the authentication and user API. Especially the `tenantId` must match the value used there, else the services can not relate data to the data of a different micro service.  With this request you create a booking tenant that does not exist before. This endpoint is probably only useful to the system itself when a new tenant is provisioned on the appointment API. If you want to change values of a booking tenant, then you have to use the `PUT` method on the existing booking tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_tenant** | [**BookingTenant**](BookingTenant.md) | the booking tenant to be created | [required] |

### Return type

[**crate::models::BookingTenant**](BookingTenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

