/* eslint-disable @typescript-eslint/no-explicit-any */
import request from './request'


export const sendShutDownCommand = ({ data, config }: { data?: any, config: any }) => {
    return request({
        baseUrl: config ? config.baseUrl : undefined,
        url: '/api/device/shutdown',
        method: 'post',
        data
    })
}

export const getDeviceStatus = ({ config }: { config: any }) => {
    return fetch(config.baseUrl + '/getStatus', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
}

export const getDeviceInfo = ({ config }: { config: any }) => {
    return fetch(config.baseUrl + '/getDeviceInfo', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
}