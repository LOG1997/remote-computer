import {
    Label,
    PolarGrid,
    PolarRadiusAxis,
    RadialBar,
    RadialBarChart,
} from "recharts"

interface Props {
    className?: string
    num: number | string
}
export function Chart(props: Props) {
    const { num = 0 } = props
    const chartData = [
        { browser: "safari", visitors: num, fill: "var(--color-safari)" },
    ]


    return (
        <div className="flex flex-col w-full">


            <RadialBarChart
                style={{ width: '100%', maxWidth: '4rem', maxHeight: '4rem', aspectRatio: 1 }}
                data={chartData}
                startAngle={0}
                endAngle={360 * Number(num) / 100}
                outerRadius={24}
                innerRadius={20}
            >
                <PolarGrid
                    gridType="circle"
                    radialLines={false}
                    stroke="none"
                    className="first:fill-muted last:fill-background "
                    polarRadius={[24, 20]}
                />
                <RadialBar fill="red" dataKey="visitors" className="" background={true} cornerRadius={10} />
                <PolarRadiusAxis tick={false} tickLine={false} axisLine={false}>
                    <Label
                        content={({ viewBox }) => {
                            if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                                return (
                                    <text
                                        x={viewBox.cx}
                                        y={viewBox.cy}
                                        textAnchor="middle"
                                        dominantBaseline="middle"
                                    >
                                        <tspan
                                            x={viewBox.cx}
                                            y={viewBox.cy}
                                            className="fill-foreground text-xs font-bold"
                                        >
                                            {Number(chartData[0].visitors).toFixed(1).toLocaleString()}%
                                        </tspan>
                                    </text>
                                )
                            }
                        }}
                    />
                </PolarRadiusAxis>
            </RadialBarChart>


        </div>
    )
}
