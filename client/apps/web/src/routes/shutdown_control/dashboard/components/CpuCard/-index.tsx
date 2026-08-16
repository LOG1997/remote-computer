import {
    Card,
    CardHeader,
    CardContent,
} from "@workspace/ui/components/card"
import { Spinner } from "@workspace/ui/components/spinner"
import { Microchip, Thermometer, AudioWaveform } from "lucide-react"
import CpuIcon from './-deviceIcon'
import { Chart } from './-chart'

interface OsProps {
    data: {
        brand: string,
        physical_core_count: string,
        total_core_count: string,
        frequency: string,
        temperature: string,
        usage: string,
    } | undefined,
    isLoading: boolean,
    className?: string
}
export default function OsCard(props: OsProps) {

    const { isLoading, data, className } = props
    return (
        <div className={className + " flex justify-center items-center min-h-32"}>
            {
                isLoading ?
                    <Card className="relative mx-auto w-full h-full max-w-sm pt-0">
                        <Spinner className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" />
                    </Card> :
                    <Card className={`relative mx-auto w-full max-w-sm pt-0 ${!data ? 'bg-muted/50 backdrop-blur-[2px] cursor-not-allowed' : ''}`}>
                        <CardHeader className="flex items-center border-b pt-4">
                            <div className='flex items-center gap-2' title={data ? data.brand : "--"}>
                                <CpuIcon cpu={data ? data.brand : "default"} />
                                <p className="truncate w-15/16 break-all">{data ? data.brand : "CPU"}</p>
                            </div>
                        </CardHeader>
                        <CardContent className="grid grid-cols-4 items-center">
                            <div id="cpu-core" className="flex flex-col gap-3">
                                <div className="flex gap-2 items-center">
                                    <Microchip size='1rem' />
                                    <p className="text-xs text-muted-foreground">核心/线程</p>
                                </div>
                                <div className="flex">
                                    <div className="text-sm font-bold">
                                        <span>{data ? data.physical_core_count : "--"}</span>
                                        <span className="text-[0.5rem] self-baseline text-muted-foreground">核</span>
                                    </div>
                                    <span>/</span>
                                    <div>
                                        <span>{data ? data.total_core_count : "--"}</span>
                                        <span className="text-[0.5rem] self-baseline text-muted-foreground">线程</span>
                                    </div>
                                </div>
                            </div>
                            <div id="cpu-frequency" className="flex flex-col gap-3 border-gray-200 pl-4">
                                <div className="flex gap-2 items-center">
                                    <AudioWaveform size='1rem' />
                                    <p className="text-xs text-muted-foreground">频率</p>
                                </div>
                                <div>
                                    <span className="text-sm font-bold">{data ? Math.floor(Number(data.frequency) / 1000).toFixed(1) : "--"}</span>
                                    <span className="text-[0.5rem] self-baseline text-muted-foreground">Ghz</span>
                                </div>
                            </div>
                            <div id="cpu-temperature" className="flex flex-col gap-3 border-gray-200 pl-4">
                                <div className="flex gap-2 items-center">
                                    <Thermometer size='1rem' />
                                    <p className="text-xs text-muted-foreground">温度</p>
                                </div>
                                <div>
                                    <span className="text-sm font-bold">{data && data.temperature ? Number(data.temperature).toFixed(1) : '--'}</span>
                                    <span>℃</span>
                                </div>
                            </div>
                            <div id="cpu-usage" className="flex flex-col gap-3 border-gray-200 pl-4">
                                <Chart num={data ? data.usage : 0} />
                            </div>
                        </CardContent>
                    </Card>
            }
        </div >
    )
}
