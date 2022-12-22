import {init_usdpl, target, init_embedded, call_backend} from "usdpl-front";

const USDPL_PORT: number = 44444;

// Utility

export function resolve(promise: Promise<any>, setter: any) {
    (async function () {
        let data = await promise;
        if (data != null) {
            console.debug("Got resolved", data);
            setter(data);
        } else {
            console.warn("Resolve failed:", data);
        }
    })();
}

export function execute(promise: Promise<any[]>) {
    (async function () {
        let data = await promise;
        console.debug("Got executed", data);
    })();
}

export async function initBackend() {
    // init usdpl
    await init_embedded();
    init_usdpl(USDPL_PORT);
    console.log("USDPL started for framework: " + target());
    //setReady(true);
}

// Back-end functions

export async function setEnabled(value: boolean): Promise<boolean> {
    return (await call_backend("set_enable", [value]))[0];
}

export async function getEnabled(): Promise<boolean> {
    return (await call_backend("get_enable", []))[0];
}

export async function setInterpolate(value: boolean): Promise<boolean> {
    return (await call_backend("set_interpolate", [value]))[0];
}

export async function getInterpolate(): Promise<boolean> {
    return (await call_backend("get_interpolate", []))[0];
}

export async function getVersion(): Promise<string> {
    return (await call_backend("version", []))[0];
}

export async function getName(): Promise<string> {
    return (await call_backend("name", []))[0];
}

export async function getCurve(): Promise<{"x": number, "y": number}[]> {
    return (await call_backend("get_curve", []))[0];
}

export async function addCurvePoint(point: {"x": number, "y": number}): Promise<{"x": number, "y": number}[]> {
    return (await call_backend("add_curve_point", [point]))[0];
}

export async function removeCurvePoint(index: number): Promise<{"x": number, "y": number}[]> {
    return (await call_backend("remove_curve_point", [index]))[0];
}

export async function getFanRpm(): Promise<number> {
    return (await call_backend("get_fan_rpm", []))[0];
}

export async function getTemperature(): Promise<number> {
    return (await call_backend("get_temperature", []))[0];
}
