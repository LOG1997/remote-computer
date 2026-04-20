import { useForm } from "@tanstack/react-form"
import { toast } from "sonner"
import * as z from "zod"
import { useState } from "react"
import { Button } from "@workspace/ui/components/button"
import { Power, RotateCw } from 'lucide-react'
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
import { useMutation } from '@tanstack/react-query'
import { Input } from "@workspace/ui/components/input"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@workspace/ui/components/dialog"
import { Checkbox } from "@workspace/ui/components/checkbox"
const ipPortRegex = /^([a-zA-Z0-9.-]+|\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d{1,5})$/

const formSchema = z.object({
    address: z
        .string()
        .min(5, "Bug title must be at least 5 characters.")
        .max(32, "Bug title must be at most 32 characters.")
        .regex(ipPortRegex, "请输入有效的 IP:端口 或 域名:端口 格式 (例如: 192.168.1.1:8080)"),
    password: z
        .string()
        .min(1, "Description must be at least 20 characters.")
        .max(100, "Description must be at most 100 characters."),
    immediate: z.boolean(),

})

export function ShutDownForm() {

    const [confirmPassword, setConfirmPassword] = useState("")
    const [isDialogOpen, setIsDialogOpen] = useState(false)
    const form = useForm({
        defaultValues: {
            address: "",
            password: "",
            immediate: true,
        },
        validators: {
            onSubmit: formSchema,
        },
    })
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
    // 3. 修改 sendShutDown：只负责验证和打开弹窗
    async function sendShutDown() {
        await form.handleSubmit()

        if (!form.state.isValid) {
            toast.error("请检查输入格式")
            return
        }

        // 重置上一次的确认密码
        setConfirmPassword("")
        // 打开弹窗
        setIsDialogOpen(true)
    }
    async function handleConfirmShutdown() {
        const originalPassword = form.getFieldValue('password')
        console.log('两次密码：', originalPassword, confirmPassword)
        if (originalPassword !== confirmPassword) {

            toast.error("两次输入的密码不一致")
            return
        }

        setIsDialogOpen(false)
        const values = {
            address: 'https://' + form.getFieldValue('address') + '/shutdown',
            data: { key: form.getFieldValue('password'), immediate: form.getFieldValue('immediate') },
        }
        try {
            await mutation.mutateAsync(values)
            toast.success("关机指令已发送")
        } catch (error) {
            console.log(error)
            toast.error("发送失败")
        }
    }

    async function sendReboot() {
        console.log('sendReboot')
    }
    return (
        <Card className="w-full sm:max-w-md px-2">
            <CardHeader>
                <CardTitle>远程关机</CardTitle>
                <CardDescription>
                    在手机操作电脑关机或者重启
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
                            name="address"
                            children={(field) => {
                                const isInvalid =
                                    field.state.meta.isTouched && !field.state.meta.isValid
                                return (
                                    <Field data-invalid={isInvalid}>
                                        <FieldLabel htmlFor={field.name}>电脑地址</FieldLabel>
                                        <Input
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            aria-invalid={isInvalid}
                                            placeholder="电脑的ip地址"
                                        // autoComplete="off"
                                        />
                                        {isInvalid && (
                                            <FieldError errors={field.state.meta.errors} />
                                        )}
                                    </Field>
                                )
                            }}
                        />
                        <form.Field
                            name="password"
                            children={(field) => {
                                const isInvalid =
                                    field.state.meta.isTouched && !field.state.meta.isValid
                                return (
                                    <Field data-invalid={isInvalid}>
                                        <FieldLabel htmlFor={field.name}>密码</FieldLabel>
                                        <Input
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            aria-invalid={isInvalid}
                                            placeholder="操作密码"
                                            autoComplete="off"
                                            type="password"
                                        />
                                        {isInvalid && (
                                            <FieldError errors={field.state.meta.errors} />
                                        )}
                                    </Field>
                                )
                            }}
                        />
                        <form.Field
                            name="immediate"
                            children={(field) => {
                                return (
                                    <div className="flex items-center space-x-2 pt-2">
                                        <Checkbox
                                            id={field.name}
                                            checked={field.state.value}
                                            onCheckedChange={(checked) => {
                                                // 确保 checked 是 boolean 类型
                                                field.handleChange(checked === true)
                                            }}
                                        />
                                        <label
                                            htmlFor={field.name}
                                            className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                                        >
                                            {field.state.value ? "立即执行" : "延迟 60s 执行"}
                                        </label>

                                    </div>
                                )
                            }}
                        />
                    </FieldGroup>
                </form>
            </CardContent>
            <CardFooter>
                <Field orientation="horizontal" className="flex w-full justify-center">
                    <Button className="cursor-pointer" type="button" size="lg" variant="destructive" onClick={sendShutDown}>
                        <Power />
                        关机
                    </Button>
                    <Button className="cursor-pointer" type="button" size="lg" variant="destructive" onClick={sendReboot}>
                        <RotateCw />
                        重启
                    </Button>
                </Field>
            </CardFooter>
            <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>安全确认</DialogTitle>
                        <DialogDescription>
                            为了安全起见，请再次输入操作密码以确认关机。
                        </DialogDescription>
                    </DialogHeader>
                    <div className="py-4">
                        <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 mb-2 block">
                            确认密码
                        </label>
                        <Input
                            type="password"
                            value={confirmPassword}
                            onChange={(e) => setConfirmPassword(e.target.value)}
                            placeholder="请再次输入密码"
                            autoFocus // 自动聚焦方便输入
                            onKeyDown={(e) => {
                                if (e.key === 'Enter') {
                                    handleConfirmShutdown()
                                }
                            }}
                        />
                    </div>
                    <DialogFooter>
                        <Button variant="outline" onClick={() => setIsDialogOpen(false)}>
                            取消
                        </Button>
                        <Button variant="destructive" onClick={handleConfirmShutdown}>
                            确认关机
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </Card>
    )
}
