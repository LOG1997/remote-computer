"use client"

import * as React from "react"
import { useForm } from "@tanstack/react-form"
import { toast } from "sonner"
import * as z from "zod"

import { Button } from "@workspace/ui/components/button"
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
import { Input } from "@workspace/ui/components/input"
import {
    InputGroup,
} from "@workspace/ui/components/input-group"

const formSchema = z.object({
    host: z.string(),
    port: z.number().int().min(1).max(65535),
    path: z.string(),
    securityKey: z.string(),
})
interface Props {
    handleSubmit: (data: any) => void
    value: any
}
export function WsConfigForm(props: Props) {
    const { handleSubmit, value } = props
    let current_host = window.location.hostname
    const form = useForm({
        defaultValues: {
            host: value && value.host || current_host || "127.0.0.1",
            port: value && value.port || 52011,
            path: value && value.path || "/user",
            securityKey: value && value.securityKey || "key",
        },
        validators: {
            onSubmit: formSchema,
        },
        onSubmit: async ({ value }) => {
            console.log('tijiao', value)
            toast("You submitted the following values:", {
                description: (
                    <pre className="mt-2 w-[320px] overflow-x-auto rounded-md bg-code p-4 text-code-foreground">
                        <code>{JSON.stringify(value, null, 2)}</code>
                    </pre>
                ),
                position: "bottom-right",
                classNames: {
                    content: "flex flex-col gap-2",
                },
                style: {
                    "--border-radius": "calc(var(--radius)  + 4px)",
                } as React.CSSProperties,
            })
            handleSubmit(value)
        },
    })

    return (
        <Card className="w-full sm:max-w-md">
            <CardHeader>
                <CardTitle>服务配置</CardTitle>
                <CardDescription>
                    请为你的服务填写信息.
                </CardDescription>
            </CardHeader>
            <CardContent>
                <form
                    id="ws-config-form"
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
                                        <FieldLabel htmlFor={field.name}>服务地址</FieldLabel>
                                        <Input
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            aria-invalid={isInvalid}
                                            placeholder="websocket服务地址"
                                            autoComplete="off"
                                        />
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
                                        <FieldLabel htmlFor={field.name}>服务端口</FieldLabel>
                                        <InputGroup>
                                            <Input
                                                type="number"
                                                id={field.name}
                                                name={field.name}
                                                value={field.state.value}
                                                onBlur={field.handleBlur}
                                                onChange={(e) => field.handleChange(Number(e.target.value))}
                                                aria-invalid={isInvalid}
                                                placeholder="websocket服务端口"
                                                autoComplete="off"
                                            />
                                        </InputGroup>
                                        {isInvalid && (
                                            <FieldError errors={field.state.meta.errors} />
                                        )}
                                    </Field>
                                )
                            }}
                        />
                        <form.Field
                            name="path"
                            children={(field) => {
                                const isInvalid =
                                    field.state.meta.isTouched && !field.state.meta.isValid
                                return (
                                    <Field data-invalid={isInvalid}>
                                        <FieldLabel htmlFor={field.name}>websocket服务接口路径</FieldLabel>
                                        <Input
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            aria-invalid={isInvalid}
                                            placeholder="websocket服务接口路径 如‘/user’"
                                            autoComplete="off"
                                        />
                                        {isInvalid && (
                                            <FieldError errors={field.state.meta.errors} />
                                        )}
                                    </Field>
                                )
                            }}
                        />
                        <form.Field
                            name="securityKey"
                            children={(field) => {
                                const isInvalid =
                                    field.state.meta.isTouched && !field.state.meta.isValid
                                return (
                                    <Field data-invalid={isInvalid}>
                                        <FieldLabel htmlFor={field.name}>密钥</FieldLabel>
                                        <InputGroup>
                                            <Input
                                                id={field.name}
                                                name={field.name}
                                                value={field.state.value}
                                                onBlur={field.handleBlur}
                                                onChange={(e) => field.handleChange(e.target.value)}
                                                aria-invalid={isInvalid}
                                                placeholder="连接websocket所需的密钥"
                                                autoComplete="off"
                                            />
                                        </InputGroup>
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
                <Field orientation="horizontal">
                    <Button type="button" variant="outline" onClick={() => form.reset()}>
                        Reset
                    </Button>
                    <Button type="submit" form="ws-config-form">
                        Submit
                    </Button>
                </Field>
            </CardFooter>
        </Card>

    )
}
