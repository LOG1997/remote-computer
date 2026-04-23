import { createFileRoute, useRouter } from '@tanstack/react-router'
import { useForm } from "@tanstack/react-form"
import * as z from "zod"
import { useState } from "react"
import { Button } from "@workspace/ui/components/button"
import { Save, CircleCheckBig, X } from 'lucide-react'
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@workspace/ui/components/card"
import {
    Field,
    FieldError,
    FieldGroup,
    FieldLabel,
} from "@workspace/ui/components/field"
import { useQuery } from '@tanstack/react-query'
import { Input } from "@workspace/ui/components/input"
import { useConfigurationStore } from '@/stores'
import { getDeviceStatus } from '@/apis';
import CustomDialog from '@/components/Dialog'
import { Spinner } from '@workspace/ui/components/spinner'
import { ButtonGroup } from "@workspace/ui/components/button-group"
export const Route = createFileRoute('/config/')({
    component: Configuration,
})

const isHostRegex = /^(localhost|[a-zA-Z0-9.-]+|\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})$/

const formSchema = z.object({
    host: z
        .string()
        .min(5, "Bug title must be at least 5 characters.")
        .max(32, "Bug title must be at most 32 characters.")
        .regex(isHostRegex, "请输入有效的 IP:端口 或 域名:端口 格式 (例如: 192.168.1.1:8080)"),
    port: z
        .string()
        .min(1, "端口不能为空")
        .max(5, "端口号无效")
        .regex(/^\d+$/, "端口必须是数字")
        .transform(Number) // 可选：转换为数字
        .refine((val) => val >= 1 && val <= 65535, { message: "端口范围必须在 1-65535 之间" }),
})
function Configuration() {
    const router = useRouter()
    const [isDialogOpen, setIsDialogOpen] = useState(false)
    const { hostname, port: localPort, protocol } = window.location
    console.log(window.location)
    console.log(location)
    // 1. 用于触发查询的状态
    const [queryUrl, setQueryUrl] = useState<string | null>(null)
    const configData = useConfigurationStore((state) => state.config)
    const setConfig = useConfigurationStore((state) => state.setConfig)
    const form = useForm({
        defaultValues: {
            host: configData?.host || hostname,
            port: configData?.port.toString() || localPort,
        },
        validators: {
            onSubmit: formSchema,
        },
    })
    // 2. 在组件顶层使用 useQuery
    const { data, isLoading, error, refetch, isSuccess, isError } = useQuery({
        queryKey: ['getStatus', queryUrl],
        queryFn: async () => {
            if (!queryUrl) throw new Error("No URL provided")
            const response = await getDeviceStatus({ config: { baseUrl: queryUrl } })
            console.log(response)
            return response
        },
        // 只有当 queryUrl 存在时才启用查询
        enabled: !!queryUrl,
        // 可选：配置重试次数等
        retry: 1,
    })
    const saveConfig = () => {
        const host = form.getFieldValue('host')
        const port = form.getFieldValue('port')
        console.log('host', host, port)
        const baseUrl = protocol + "//" + host + ":" + port
        setQueryUrl(baseUrl)
        setIsDialogOpen(true)

    }
    const setDefaultConfig = () => {
        form.setFieldValue('host', hostname)
        form.setFieldValue('port', localPort)
    }
    const handleConfirm = () => {
        setIsDialogOpen(false)
        const host = form.getFieldValue('host')
        const port = form.getFieldValue('port')
        setConfig({
            host,
            port
        })
        // 路由跳转
        router.navigate({ to: '/dashboard', replace: true })
    }
    return (
        <div className='flex justify-center'>
            <Card className="w-full sm:max-w-md px-2">
                <CardHeader>
                    <CardTitle>配置地址</CardTitle>
                    <CardDescription>
                        输入您的电脑的IP地址和端口
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <form
                        id="bug-report-form"
                        onSubmit={(e) => {
                            e.preventDefault()
                            form.handleSubmit()
                        }}
                    >
                        <FieldGroup>
                            <form.Field
                                name="host"
                                children={(field) => {
                                    const isInvalid =
                                        field.state.meta.isTouched && !field.state.meta.isValid
                                    return (
                                        <Field data-invalid={isInvalid}>
                                            <FieldLabel htmlFor={field.name}>电脑地址</FieldLabel>

                                            <ButtonGroup>
                                                <Input
                                                    id={field.name}
                                                    name={field.name}
                                                    value={field.state.value}
                                                    onBlur={field.handleBlur}
                                                    onChange={(e) => field.handleChange(e.target.value)}
                                                    aria-invalid={isInvalid}
                                                    placeholder="电脑的ip地址"
                                                />
                                                <Button onClick={setDefaultConfig}>使用默认地址</Button>
                                            </ButtonGroup>

                                            {isInvalid && (
                                                <FieldError errors={field.state.meta.errors} />
                                            )}
                                        </Field>
                                    )
                                }}
                            />
                            <form.Field
                                name="port"
                                children={(field) => {
                                    const isInvalid =
                                        field.state.meta.isTouched && !field.state.meta.isValid
                                    return (
                                        <Field data-invalid={isInvalid}>
                                            <FieldLabel htmlFor={field.name}>端口</FieldLabel>
                                            <Input
                                                id={field.name}
                                                name={field.name}
                                                value={field.state.value}
                                                onBlur={field.handleBlur}
                                                onChange={(e) => field.handleChange(e.target.value)}
                                                aria-invalid={isInvalid}
                                                placeholder="电脑监听的端口号"
                                            // type='number'
                                            />
                                            {isInvalid && (
                                                <FieldError errors={field.state.meta.errors} />
                                            )}
                                        </Field>
                                    )
                                }}
                            />
                        </FieldGroup>
                    </form>
                </CardContent>
                <CardFooter>
                    <Field orientation="horizontal" className="flex w-full justify-center">
                        <Button className="cursor-pointer" type="button" size="lg" onClick={saveConfig}>
                            <Save />
                            保存
                        </Button>
                    </Field>
                </CardFooter>
            </Card>
            <CustomDialog open={isDialogOpen} setOpen={setIsDialogOpen} handleConfirm={handleConfirm} desc='设备状态如下'>
                <div>
                    {isLoading ? (
                        <div className="flex flex-col items-center gap-2">
                            <Spinner />
                            <p className="text-sm text-muted-foreground">正在连接设备...</p>
                        </div>
                    ) : isError ? (
                        <div className="flex flex-col items-center gap-2 text-destructive">
                            <p className="font-medium">连接失败</p>
                            <p>无法获取设备状态，请检查地址和端口，您仍可以保存配置</p>
                            <p className="text-sm">{error.message || ""}</p>
                            <Button variant="outline" size="sm" onClick={() => refetch()}>
                                重试
                            </Button>
                        </div>
                    ) : isSuccess && data && data.success ? (
                        <div className="flex flex-col items-center gap-2 text-green-600 dark:text-green-500">
                            <p className="font-medium">连接成功</p>
                            <CircleCheckBig className='text-green-500' />
                        </div >
                    ) : (
                        <div className="flex flex-col items-center gap-2 text-red-600 dark:text-red-500">
                            <p className="font-medium">连接失败</p>
                            <X className='text-red-500' />
                            <p className="text-sm text-gray-300/70">您仍可以保存配置</p>
                        </div >
                    )
                    }
                </div >
            </CustomDialog >
        </div>
    )
}
