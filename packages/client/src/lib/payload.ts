export interface Point2<T> {
    x: T;
    y: T;
}

export interface Rect {
    top_left: Point2<number>;
    bottom_right: Point2<number>;
}

export type TextSize = "Small" | "Normal" | "Large";
export type DisplayRenderMode = "Immediate" | "DoubleBuffered";
export type TextLocation =
    | {
          Coordinates: Point2<number>;
      }
    | {
          Line: number;
      };
export type ScrollLocation =
    | {
          Rect: Rect;
      }
    | {
          Line: number;
      };

export type Shape =
    | {
          Rectangle: {
              top_left: Point2<number>;
              bottom_right: Point2<number>;
          };
      }
    | {
          Circle: {
              center: Point2<number>;
              radius: number;
          };
      }
    | {
          Line: {
              start: Point2<number>;
              end: Point2<number>;
          };
      };

export type DrawCommand =
    | {
          Fill: Shape;
      }
    | {
          Stroke: Shape;
      }
    | {
          CopyBuffer: {
              top_left: Point2<number>;
              bottom_right: Point2<number>;
              stride: number;
              buffer: number[];
          };
      }
    | {
          Text: {
              data: string;
              size: TextSize;
              location: TextLocation;
              opaque: boolean;
              background: number;
          };
      };

export interface DisplayDrawPayload {
    command: DrawCommand;
    color: number;
    clip_region: Rect;
}

export interface DisplayClearPayload {
    color: number;
    clip_region: Rect;
}

export interface DisplayScrollPayload {
    location: ScrollLocation;
    lines: number;
    background: number;
    clip_region: Rect;
}

// Tauri drag event payloads

export type DragDropPayload = {
    paths: string[];
    position: { x: number; y: number };
};
export type DragEnterPayload = DragDropPayload;
export type DragOverPayload = {
    position: { x: number; y: number };
};

// Node Graph interpreter payloads

export type NodeGraphDevice =
    | {
        DistanceSensor: { distance: number; size: number };
    }
    | {
        LightSensor: { darkness: number }
    }

export type NodeGraphUpdatePayload = {
    devices: { [id: string]: NodeGraphDevice }
};
