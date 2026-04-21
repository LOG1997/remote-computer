
import { createFileRoute } from '@tanstack/react-router'
import { Badge } from "@workspace/ui/components/badge"
import { Button } from "@workspace/ui/components/button"
import {
    Card,
    CardAction,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@workspace/ui/components/card"
import { useQuery } from "@tanstack/react-query"
import { getDeviceStatus, getDeviceInfo } from '@/apis'
import { useConfigurationStore } from '@/stores'
export const Route = createFileRoute('/dashboard/')({
    component: Dashboard,
})
function Dashboard() {
    const configData = useConfigurationStore((state) => state.config)
    const { protocol } = window.location
    const baseUrl = protocol + "//" + configData?.host + ":" + configData?.port
    const { data, isLoading, error, refetch, isSuccess, isError } = useQuery({
        queryKey: ['deviceStatus', baseUrl],
        queryFn: async () => {
            if (!baseUrl) throw new Error("No URL provided")
            const response = await getDeviceInfo({ config: { baseUrl } })
            console.log(response)
            return response
        },
        // 只有当 queryUrl 存在时才启用查询
        enabled: !!baseUrl,
        // 可选：配置重试次数等
        retry: 1,
    })
    console.log(data);
    return (
        <div>
            <Card className="relative mx-auto w-full max-w-sm pt-0">
                <div className="absolute inset-0 z-30 aspect-video bg-black/35" />
                <img
                    src="https://avatar.vercel.sh/shadcn1"
                    alt="Event cover"
                    className="relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40"
                />
                <CardHeader>getDeviceStatus
                    <CardAction>
                        <Badge variant="secondary">Featured</Badge>
                    </CardAction>
                    <CardTitle>Design systems meetup</CardTitle>
                    <CardDescription>
                        A practical talk on component APIs, accessibility, and shipping
                        faster.
                    </CardDescription>
                </CardHeader>
                <CardFooter>
                    <Button className="w-full">View Event</Button>
                </CardFooter>
            </Card>
        </div>
    )
}
