import { Input } from "@workspace/ui/components/input"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@workspace/ui/components/dialog"
import * as z from "zod"
import { Button } from "@workspace/ui/components/button"
import { Checkbox } from "@workspace/ui/components/checkbox"
import {
    Field,
    FieldError,
    FieldGroup,
    FieldLabel,
} from "@workspace/ui/components/field"
import { useForm } from "@tanstack/react-form"
import { toast } from "sonner"

interface Props {
    mode: 'reboot' | 'shutdown';
    open: boolean;
    setOpen: (open: boolean) => void;
    handleSubmit: (values: { password: string, immediate: boolean }) => void;
}
const formSchema = z.object({
    password: z
        .string()
        .min(1, "密码不能为空"),
    immediate: z.boolean(),

})
export default function ShutDownDialog(props: Props) {

    const { open, setOpen, mode = 'shutdown', handleSubmit } = props; const form = useForm({
        defaultValues: {
            password: "",
            immediate: true,
        },
        validators: {
            onSubmit: formSchema,
        },
    })

    const handleDialogConfirm = async () => {
        // 验证表单
        await form.handleSubmit()
        if (!form.state.isValid) {
            toast.error("请检查输入格式")
            return
        }
        const formValues = form.state.values
        console.log('fommmra:', formValues)
        handleSubmit(formValues)
    }
    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{mode === 'reboot' ? '重启' : '关机'}安全确认</DialogTitle>
                    <DialogDescription>
                        为了安全起见，请输入操作密码以确认{mode === 'reboot' ? '重启' : '关机'}。
                    </DialogDescription>
                </DialogHeader>
                <form
                    id="bug-report-form"
                    onSubmit={(e) => {
                        e.preventDefault()
                        form.handleSubmit()
                    }}
                >
                    <FieldGroup>
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
                                            立即执行
                                        </label>

                                    </div>
                                )
                            }}
                        />
                    </FieldGroup>
                </form>
                <DialogFooter>
                    <Button variant="outline" onClick={() => setOpen(false)}>
                        取消
                    </Button>
                    <Button variant="destructive" onClick={handleDialogConfirm}>
                        确认
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}
