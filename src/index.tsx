import {
  definePlugin,
  DialogButton,
  PanelSection,
  PanelSectionRow,
  ServerAPI,
  ToggleField,
  staticClasses,
  gamepadDialogClasses,
  joinClassNames,
} from "decky-frontend-lib";
import { VFC, useState } from "react";
import { FaFan } from "react-icons/fa";
import { SiOnlyfans } from "react-icons/si";

import * as backend from "./backend";
import {Canvas} from "./canvas";

const POINT_SIZE = 32;

var periodicHook: any = null;
var usdplReady: boolean = false;

var curve_backup: {x: number, y: number}[] = [];

const Content: VFC<{ serverAPI: ServerAPI }> = ({serverAPI}) => {
  // const [result, setResult] = useState<number | undefined>();

  // const onClick = async () => {
  //   const result = await serverAPI.callPluginMethod<AddMethodArgs, number>(
  //     "add",
  //     {
  //       left: 2,
  //       right: 2,
  //     }
  //   );
  //   if (result.success) {
  //     setResult(result.result);
  //   }
  // };

  const [enabledGlobal, setEnableInternal] = useState<boolean>(false);
  const [interpolGlobal, setInterpol] = useState<boolean>(false);
  const [_serverApiGlobal, setServerApi] = useState<ServerAPI>(serverAPI);
  const [firstTime, setFirstTime] = useState<boolean>(true);
  const [curveGlobal, setCurve_internal] = useState<{x: number, y: number}[]>(curve_backup);

  const setCurve = (value: {x: number, y: number}[]) => {
    setCurve_internal(value);
    curve_backup = value;
  }

  const [temperatureGlobal, setTemperature] = useState<number>(-273.15);
  const [fanRpmGlobal, setFanRpm] = useState<number>(-1337);

  function setEnable(enable: boolean) {
    setEnableInternal(enable);
  }

  function onClickCanvas(e: any) {
    //console.log("canvas click", e);
    const realEvent: any = e.nativeEvent;
    //console.log("Canvas click @ (" + realEvent.layerX.toString() + ", " + realEvent.layerY.toString() + ")");
    const target: any = e.currentTarget;
    //console.log("Target dimensions " + target.width.toString() + "x" + target.height.toString());
    const clickX = realEvent.layerX;
    const clickY = realEvent.layerY;
    for (let i = 0; i < curveGlobal.length; i++) {
      const curvePoint = curveGlobal[i];
      const pointX = curvePoint.x * target.width;
      const pointY = (1 - curvePoint.y) * target.height;
      if (
        pointX + POINT_SIZE > clickX
        && pointX - POINT_SIZE < clickX
        && pointY + POINT_SIZE > clickY
        && pointY - POINT_SIZE < clickY
        ) {
          //console.log("Clicked on point " + i.toString());
          backend.resolve(backend.removeCurvePoint(i), setCurve);
          return;
        }
    }
    //console.log("Adding new point");
    backend.resolve(backend.addCurvePoint({x: clickX / target.width, y: 1 - (clickY / target.height)}), setCurve);
  }

  function drawCanvas(ctx: any, frameCount: number): void {
    if (frameCount % 100 > 1) {
      return;
    }
    const width: number = ctx.canvas.width;
    const height: number = ctx.canvas.height;

    ctx.strokeStyle = "#1a9fff";
    ctx.fillStyle = "#1a9fff";
    ctx.lineWidth = 2;
    ctx.lineJoin = "round";
    //ctx.beginPath();
    ctx.clearRect(0, 0, width, height);
    /*ctx.arc(75, 75, 50, 0, Math.PI * 2, true); // Outer circle
    ctx.moveTo(110, 75);
    ctx.arc(75, 75, 35, 0, Math.PI, false);  // Mouth (clockwise)
    ctx.moveTo(65, 65);
    ctx.arc(60, 65, 5, 0, Math.PI * 2, true);  // Left eye
    ctx.moveTo(95, 65);
    ctx.arc(90, 65, 5, 0, Math.PI * 2, true);  // Right eye*/
    //ctx.beginPath();
    //ctx.moveTo(0, height);
    if (interpolGlobal) {
      ctx.beginPath();
      ctx.moveTo(0, height);
      for (let i = 0; i < curveGlobal.length; i++) {
        const canvasHeight = (1 - curveGlobal[i].y) * height;
        const canvasWidth = curveGlobal[i].x * width;
        ctx.lineTo(canvasWidth, canvasHeight);
        ctx.moveTo(canvasWidth, canvasHeight);
        ctx.arc(canvasWidth, canvasHeight, 8, 0, Math.PI * 2);
        ctx.moveTo(canvasWidth, canvasHeight);
      }
      ctx.lineTo(width, 0);
      //ctx.moveTo(width, 0);
      ctx.stroke();
      ctx.fill();
    } else {
      ctx.beginPath();
      ctx.moveTo(0, height);
      for (let i = 0; i < curveGlobal.length - 1; i++) {
        const canvasHeight = (1 - curveGlobal[i].y) * height;
        const canvasWidth = curveGlobal[i].x * width;
        const canvasHeight2 = (1 - curveGlobal[i+1].y) * height;
        const canvasWidth2 = curveGlobal[i+1].x * width;
        //ctx.lineTo(canvasWidth, canvasHeight);
        ctx.moveTo(canvasWidth, canvasHeight);
        ctx.arc(canvasWidth, canvasHeight, 8, 0, Math.PI * 2);
        ctx.moveTo(canvasWidth, canvasHeight);
        ctx.lineTo(canvasWidth2, canvasHeight);
        ctx.moveTo(canvasWidth2, canvasHeight);
        ctx.lineTo(canvasWidth2, canvasHeight2);
      }
      if (curveGlobal.length != 0) {
        const i = curveGlobal.length - 1;
        const canvasHeight = (1 - curveGlobal[i].y) * height;
        const canvasWidth = curveGlobal[i].x * width;
        //ctx.lineTo(width, 0);
        ctx.moveTo(canvasWidth, canvasHeight);
        ctx.arc(canvasWidth, canvasHeight, 8, 0, Math.PI * 2);
        ctx.moveTo(canvasWidth, canvasHeight);
        ctx.lineTo(width, canvasHeight);
        //ctx.moveTo(width, canvasHeight);
        //ctx.lineTo(width, 0);
        const canvasHeight2 = (1 - curveGlobal[0].y) * height;
        const canvasWidth2 = curveGlobal[0].x * width;
        ctx.moveTo(canvasWidth2, canvasHeight2);
        ctx.lineTo(canvasWidth2, height);
      }

      //ctx.moveTo(width, 0);
      ctx.stroke();
      ctx.fill();
    }
    console.debug("Rendered fan graph canvas frame", frameCount);
    //console.debug("Drew canvas with " + curveGlobal.length.toString() + " points; " + width.toString() + "x" + height.toString());
    //ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    //ctx.fillStyle = '#000000';
    //ctx.beginPath();
    //ctx.arc(50, 100, 20*Math.sin(frameCount*0.05)**2, 0, 2*Math.PI);
    //ctx.fill();

  }

  if (firstTime) {
    setFirstTime(false);
    setServerApi(serverAPI);
    backend.resolve(backend.getEnabled(), setEnable);
    backend.resolve(backend.getInterpolate(), setInterpol);
    backend.resolve(backend.getCurve(), setCurve);
    backend.resolve(backend.getTemperature(), setTemperature);
    backend.resolve(backend.getFanRpm(), setFanRpm);

    if (periodicHook != null) {
      clearInterval(periodicHook);
    }

    periodicHook = setInterval(function() {
        backend.resolve(backend.getTemperature(), setTemperature);
        backend.resolve(backend.getFanRpm(), setFanRpm);
    }, 1000);
  }

  if (!usdplReady) {
    return (
      <PanelSection>
      </PanelSection>
    );
  }

  const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

  // TODO handle clicking on fan curve nodes

  return (
    <PanelSection>
      <PanelSectionRow>
        <div className={FieldWithSeparator}>
          <div className={gamepadDialogClasses.FieldLabelRow}>
            <div className={gamepadDialogClasses.FieldLabel}>
            Current Fan Speed
            </div>
            <div className={gamepadDialogClasses.FieldChildren}>
            {fanRpmGlobal.toFixed(0) + " RPM"}
            </div>
          </div>
        </div>
      </PanelSectionRow>
      <PanelSectionRow>
        <div className={FieldWithSeparator}>
          <div className={gamepadDialogClasses.FieldLabelRow}>
            <div className={gamepadDialogClasses.FieldLabel}>
            Current Temperature
            </div>
            <div className={gamepadDialogClasses.FieldChildren}>
            {temperatureGlobal.toFixed(1) + " °C"}
            </div>
          </div>
        </div>
      </PanelSectionRow>
      <PanelSectionRow>
        <ToggleField
          label="Custom Fan Curve"
          description="Overrides SteamOS fan curve"
          checked={enabledGlobal}
          onChange={(value: boolean) => {
            backend.resolve(backend.setEnabled(value), setEnable);
          }}
        />
      </PanelSectionRow>
      { enabledGlobal &&
      <div className={staticClasses.PanelSectionTitle}>
        Fan
      </div>
      }
      { enabledGlobal &&
      <PanelSectionRow>
        <Canvas draw={drawCanvas} width={268} height={200} style={{
          "width": "268px",
          "height": "200px",
          "padding":"0px",
          "border":"1px solid #1a9fff",
          //"position":"relative",
          "background-color":"#1a1f2c",
          "border-radius":"4px",
          //"margin":"auto",
        }} onClick={(e: any) => onClickCanvas(e)}/>
      </PanelSectionRow>
      }
      { enabledGlobal &&
      <PanelSectionRow>
        <ToggleField
          label="Linear Interpolation"
          description="Pretends a straight line connects points"
          checked={interpolGlobal}
          onChange={(value: boolean) => {
            backend.resolve(backend.setInterpolate(value), setInterpol);
          }}
        />
      </PanelSectionRow>
      }
    </PanelSection>
  );
};

const DeckyPluginRouterTest: VFC = () => {
  return (
    <div style={{ marginTop: "50px", color: "white" }}>
      Hello World!
      <DialogButton onClick={() => {}}>
        Go to Store
      </DialogButton>
    </div>
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute("/decky-plugin-test", DeckyPluginRouterTest, {
    exact: true,
  });

  (async function(){
      await backend.initBackend();
      usdplReady = true;
      backend.getEnabled();
    })();

  let ico = <FaFan />;
  let now = new Date();
  if (now.getDate() == 1 && now.getMonth() == 3) {
    ico = <SiOnlyfans/>;
  }

  return {
    title: <div className={staticClasses.Title}>Fantastic</div>,
    content: <Content serverAPI={serverApi} />,
    icon: ico,
    onDismount() {
      clearInterval(periodicHook!);
      serverApi.routerHook.removeRoute("/decky-plugin-test");
    },
  };
});
