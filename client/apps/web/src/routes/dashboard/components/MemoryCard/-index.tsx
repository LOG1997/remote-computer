import {
    Card,
    CardHeader,
    CardContent,
} from "@workspace/ui/components/card"
import { Spinner } from "@workspace/ui/components/spinner"
import { MemoryStick } from "lucide-react"
import { Chart } from './-chart'

interface MemoryProps {
    data: {
        total_memory: string,
        used_memory: string,
        free_memory: string,
    } | undefined,
    isLoading: boolean,
    className?: string
}
export default function OsCard(props: MemoryProps) {

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
                            <div className='flex items-center gap-2'>
                                <MemoryStick />
                                <p className="truncate w-2/3">内存</p>
                            </div>
                        </CardHeader>
                        <CardContent className="grid grid-cols-3">
                            <div id="memory-used" className="flex flex-col gap-3">
                                <div className="flex gap-2 items-center">
                                    <p className="text-xs text-muted-foreground">已用</p>
                                </div>
                                <div className="flex">
                                    <div className="text-sm font-bold">
                                        <span>{(Number(data.used_memory) / (1024 * 1024 * 1024)).toFixed(1)}</span>
                                        <span className="text-[0.5rem] self-baseline text-muted-foreground">GB</span>
                                    </div>
                                </div>
                            </div>
                            <div id="cpu-frequency" className="flex flex-col gap-3 border-gray-200 pl-4">
                                <div className="flex gap-2 items-center">
                                    <p className="text-xs text-muted-foreground">可用</p>
                                </div>
                                <div>
                                    <span className="text-sm font-bold">{(Number(data.free_memory) / (1024 * 1024 * 1024)).toFixed(1)}</span>
                                    <span className="text-[0.5rem] self-baseline text-muted-foreground">GB</span>
                                </div>
                            </div>
                            <div id="cpu-usage" className="flex flex-col gap-3 border-gray-200 pl-4">
                                <Chart usedMemory={data.used_memory} totalMemory={data.total_memory} />
                            </div>
                        </CardContent>
                    </Card>
            }
        </div >
    )
}
