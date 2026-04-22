import {
    Card,
    CardHeader,
    CardContent,
} from "@workspace/ui/components/card"
import { Spinner } from "@workspace/ui/components/spinner"
import { Button } from "@workspace/ui/components/button"
import { Dot } from "lucide-react"
import { useMutation } from '@tanstack/react-query'
interface OsProps {
    data: boolean,
    isLoading: boolean,
    className?: string
}
export default function OsCard(props: OsProps) {
    const { isLoading, data, className } = props
    const mutation = useMutation({
        mutationFn: async (values: { address: string; data: object }) => {
            const { address, data } = values
            return fetch(address, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data),
            })
        },
    })
    return (
        <div className={className + " flex justify-center items-center min-h-18"}>
            {
                isLoading ?
                    <Card className="relative mx-auto w-full h-full max-w-sm pt-0">
                        <Spinner className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" />
                    </Card> :
                    <Card className="relative mx-auto w-full max-w-sm pt-0">
                        <CardContent className="flex justify-between items-center pt-3">
                            <div id="os-system" className=" flex flex-col gap-3 justify-center">
                                {
                                    data ?
                                        <div className="flex gap-1 items-center">
                                            <Dot className="text-green-500" strokeWidth={8} />
                                            <span>在线</span>
                                        </div> :
                                        <div className="flex gap-1 items-center">
                                            <Dot className="text-red-500" strokeWidth={8} />
                                            <span>离线</span>
                                        </div>
                                }
                            </div>
                            <div id="shut-down" className="pl-12 flex gap-2">
                                <Button variant="destructive">关机</Button>
                                <Button variant="outline">重启</Button>
                            </div>

                        </CardContent>
                    </Card>
            }

        </div >
    )
}
