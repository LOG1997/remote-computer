
import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from "@tanstack/react-query"
import { getDeviceStatus, getDeviceInfo } from '@/apis'
import { useConfigurationStore } from '@/stores'
import OsCard from "./components/OsCard/-index"
import CpuCard from "./components/CpuCard/-index"
import MemoryCard from "./components/MemoryCard/-index"
import StatusCard from "./components/StatusCard/-index"

export const Route = createFileRoute('/dashboard/')({
    component: Dashboard,
})
function Dashboard() {
    const configData = useConfigurationStore((state) => state.config)
    const { protocol } = window.location
    const baseUrl = protocol + "//" + configData?.host + ":" + configData?.port
    const { data: deviceData, isLoading } = useQuery({
        queryKey: ['deviceInfo', baseUrl],
        queryFn: async () => {
            if (!baseUrl) throw new Error("No URL provided")
            const response = await getDeviceInfo({ config: { baseUrl } })
            console.log('response:', response);
            return response.data
        },
        // 只有当 queryUrl 存在时才启用查询
        enabled: !!baseUrl,
        // 可选：配置重试次数等
        retry: 1,
        refetchInterval: 5000,
    })
    const { data: statusData } = useQuery({
        queryKey: ['deviceStatus', baseUrl],
        queryFn: async () => {
            if (!baseUrl) throw new Error("No URL provided")
            const response = await getDeviceStatus({ config: { baseUrl } })
            return response
        },
        // 只有当 queryUrl 存在时才启用查询
        enabled: !!baseUrl,
        // 可选：配置重试次数等
        retry: 1,
        refetchInterval: 5000,
    })
    return (
        <div className='flex flex-col gap-8'>
            <StatusCard data={statusData?.success} isLoading={isLoading} className='h-18' />
            <OsCard data={deviceData?.os} isLoading={isLoading} className='h-32' />
            <CpuCard data={deviceData?.cpu} isLoading={isLoading} className='h-32' />
            <MemoryCard data={deviceData?.memory} isLoading={isLoading} className='h-32' />
        </div>
    )
}
