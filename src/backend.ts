//import {init_usdpl, target_usdpl, init_embedded, call_backend} from "usdpl-front";

import { init_embedded, target_usdpl } from "fantastic-wasm";
import { Fan } from "fantastic-wasm";

//@ts-ignore
//const Fan = {};

const USDPL_PORT: number = 44444;

var FAN_CLIENT: Fan | undefined = undefined;

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
    FAN_CLIENT = new Fan(USDPL_PORT);
    console.log("FANTASTIC: USDPL started for framework: " + target_usdpl());
    //setReady(true);
}

// Back-end functions

export async function setEnabled(value: boolean): Promise<boolean> {
    return (await FAN_CLIENT!.set_enable(value))?? value;
    //return (await call_backend("set_enable", [value]))[0];
}

export async function getEnabled(): Promise<boolean> {
    return (await FAN_CLIENT!.get_enable(true)) ?? false;
}

export async function setInterpolate(value: boolean): Promise<boolean> {
    return (await FAN_CLIENT!.set_interpolate(value)) ?? value;
    //return (await call_backend("set_interpolate", [value]))[0];
}

export async function getInterpolate(): Promise<boolean> {
    return (await FAN_CLIENT!.get_interpolate(true)) ?? false;
    //return (await call_backend("get_interpolate", []))[0];
}

export async function getVersion(): Promise<string> {
    return (await FAN_CLIENT!.version_str(true)) ?? "version";
    //return (await call_backend("version", []))[0];
}

export async function getName(): Promise<string> {
    return (await FAN_CLIENT!.name(true))?? "broken";
    //return (await call_backend("name", []))[0];
}

export async function getCurve(): Promise<{"x": number, "y": number}[]> {
    var x_s = (await FAN_CLIENT!.get_curve_x(true))?? [];
    var y_s = (await FAN_CLIENT!.get_curve_y(true))?? [];
    let result: {"x": number, "y": number}[] = [];
    for (let i = 0; i < x_s.length && i < y_s.length; i++) {
        result.push({
            x: x_s[i],
            y: y_s[i],
        });
    }
    return result;
}

export async function addCurvePoint(point: {"x": number, "y": number}): Promise<{"x": number, "y": number}[]> {
    await FAN_CLIENT!.add_curve_point(point.x, point.y);
    return getCurve();
}

export async function removeCurvePoint(index: number): Promise<{"x": number, "y": number}[]> {
    await FAN_CLIENT!.remove_curve_point(index);
    return getCurve();
    //return (await call_backend("remove_curve_point", [index]))[0];
}

export async function getFanRpm(): Promise<number> {
    return (await FAN_CLIENT!.get_fan_rpm(true))?? 1337;
    //return (await call_backend("get_fan_rpm", []))[0];
}

export async function getTemperature(): Promise<number> {
    return (await FAN_CLIENT!.get_temperature(true))?? -273;
    //return (await call_backend("get_temperature", []))[0];
}

initBackend();
