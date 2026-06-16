import { Progress } from "@workspace/ui/components/progress"
import { Slider } from "@workspace/ui/components/slider"
import { useState } from "react"

function VolumeAction() {
    const [value, setValue] = useState([50])
    return (
        <div className="flex w-full max-w-sm flex-col gap-4" >
            <Progress value={value[0]} />
            <Slider
                value={value}
                onValueChange={setValue}
                min={0}
                max={100}
                step={1}
            />
        </div >
    )
}
