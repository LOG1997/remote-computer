import {
    Card,
    CardHeader,
    CardContent,
} from "@workspace/ui/components/card"
import DeviceIcon from "./-deviceIcon"
import { Spinner } from "@workspace/ui/components/spinner"
import { Server, Tag } from "lucide-react"

interface OsProps {
    data: {
        platform: string,
        host_name: string,
        name: string,
        os_version: string,
        kernel_version: string,
    } | undefined,
    isLoading: boolean,
    className?: string
}
export default function OsCard(props: OsProps) {
    const { isLoading, data, className } = props
    return (
        <div className={className + " flex justify-center items-center min-h-32"}>
            {
                isLoading || !data ?
                    <Card className="relative mx-auto w-full h-full max-w-sm pt-0">
                        <Spinner className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" />
                    </Card> :
                    <Card className="relative mx-auto w-full max-w-sm pt-0">
                        <CardHeader className="flex items-center border-b pt-4">
                            <div className='flex items-center gap-2' title={data.host_name}>
                                <DeviceIcon device={data?.platform} />
                                <p className="truncate w-15/16 break-all">{data.host_name}</p>
                            </div>
                            {/* TODO:展示别的状态 */}
                            <div>

                            </div>
                        </CardHeader>
                        <CardContent className="grid grid-cols-2 gap-4">
                            <div id="os-system" className="flex flex-col gap-3">
                                <div className="flex gap-2 items-center">
                                    <Server size='1rem' />
                                    <p className="text-xs text-muted-foreground">操作系统</p>
                                </div>
                                <p className="text-sm font-bold">{data.name}</p>
                            </div>
                            <div id="os-version" className="flex flex-col gap-3 border-l border-gray-200 pl-4">
                                <div className="flex gap-2 items-center">
                                    <Tag size='1rem' />
                                    <p className="text-xs text-muted-foreground">系统版本</p>
                                </div>
                                <p className="text-sm font-bold">{data.os_version}</p>
                            </div>
                        </CardContent>
                    </Card>
            }

        </div>
    )
}
