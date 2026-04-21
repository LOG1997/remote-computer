
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
import { getDeviceStatus } from '@/apis'
import { useConfigurationStore } from '@/stores'
export const Route = createFileRoute('/dashboard/')({
    component: Dashboard,
})
function Dashboard() {
    const configData = useConfigurationStore((state) => state.config)
    const statusInfo = useQuery({ queryKey: ['deviceStatus'], queryFn: () => getDeviceStatus({ config: { baseUrl: "https://" + configData?.ip + ":" + configData?.port } }) })
    console.log(statusInfo);
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
