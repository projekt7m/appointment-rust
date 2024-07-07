# \BookingTenantApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_booking_tenant_by_id**](BookingTenantApi.md#delete_booking_tenant_by_id) | **DELETE** /booking/tenant/{id} | Delete a booking tenant by ID
[**get_booking_tenant_by_id**](BookingTenantApi.md#get_booking_tenant_by_id) | **GET** /booking/tenant/{id} | Get a (booking) tenant as specified by its ID
[**get_booking_tenants**](BookingTenantApi.md#get_booking_tenants) | **GET** /booking/tenant | Get list of booking tenants
[**post_booking_tenant**](BookingTenantApi.md#post_booking_tenant) | **POST** /booking/tenant | Create a new booking tenant
[**put_booking_tenant_by_id**](BookingTenantApi.md#put_booking_tenant_by_id) | **PUT** /booking/tenant/{id} | Update an existing booking tenant



## delete_booking_tenant_by_id

> delete_booking_tenant_by_id(id)
Delete a booking tenant by ID

Delete a booking tenant by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the booking tenant | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_booking_tenant_by_id

> crate::models::BookingTenant get_booking_tenant_by_id(id)
Get a (booking) tenant as specified by its ID

Get a (booking) tenant as specified by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the (booking) tenant | [required] |

### Return type

[**crate::models::BookingTenant**](BookingTenant.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_booking_tenants

> crate::models::BookingTenantData get_booking_tenants()
Get list of booking tenants

Get list of booking tenants  A booking tenant is a tenant as the appointment API sees it.  Data her eshould more or less mirror the information of the Tenant as it is configured in the authentication and user API. Especially the `tenantId` must match the value used there, else the services cannot relate data to the data of a different micro service.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BookingTenantData**](BookingTenantData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_booking_tenant

> crate::models::BookingTenant post_booking_tenant(booking_tenant)
Create a new booking tenant

Create a new booking tenant  A booking tenant is a tenant as the appointment API sees it.  Data here should more or less mirror the information of the Tenant as it is configured in the authentication and user API. Especially the `tenantId` must match the value used there, else the services cannot relate data to the data of a different micro service.  With this request you create a booking tenant that does not exist before. This endpoint is probably only useful to the system itself when a new tenant is provisioned on the appointment API. If you want to change values of a booking tenant, then you have to use the `PUT` method on the existing booking tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_tenant** | [**BookingTenant**](BookingTenant.md) | The booking tenant to be created | [required] |

### Return type

[**crate::models::BookingTenant**](BookingTenant.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_booking_tenant_by_id

> crate::models::BookingTenant put_booking_tenant_by_id(id, booking_tenant)
Update an existing booking tenant

Update an existing booking tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the (booking) tenant | [required] |
**booking_tenant** | [**BookingTenant**](BookingTenant.md) | The updated booking tenant | [required] |

### Return type

[**crate::models::BookingTenant**](BookingTenant.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

