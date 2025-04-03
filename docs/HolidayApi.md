# \HolidayApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_holidays**](HolidayApi.md#get_holidays) | **GET** /holidays | Get the list of public holidays for catholic regions of Bavaria
[**get_holidays_regions**](HolidayApi.md#get_holidays_regions) | **GET** /holidays/regions | Get the list of holiday regions known by the system
[**get_holidays_regions_holiday_region_id**](HolidayApi.md#get_holidays_regions_holiday_region_id) | **GET** /holidays/regions/{hrid} | Get the holidays for a given region



## get_holidays

> models::ListWrapperHoliday get_holidays()
Get the list of public holidays for catholic regions of Bavaria

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListWrapperHoliday**](ListWrapper_Holiday.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_holidays_regions

> models::ListWrapperHolidayRegion get_holidays_regions()
Get the list of holiday regions known by the system

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListWrapperHolidayRegion**](ListWrapper_HolidayRegion.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_holidays_regions_holiday_region_id

> models::ListWrapperHoliday get_holidays_regions_holiday_region_id(hrid)
Get the holidays for a given region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hrid** | **uuid::Uuid** | The region to get holidays for | [required] |

### Return type

[**models::ListWrapperHoliday**](ListWrapper_Holiday.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

