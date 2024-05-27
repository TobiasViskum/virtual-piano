import { Fragment, useEffect, useRef, useState } from "react";

let next_key = 21;
const activeColor = "red";

function getNextKey(): number {
  let key = next_key;

  next_key++;
  return key;
}

export function PianoView() {
  return (
    <div className="relative overflow-hidden min-h-[160px] flex">
      <StartSection />
      <CESection />
      <FBSection />
      <CESection />
      <FBSection />
      <CESection />
      <FBSection />
      <CESection />
      <FBSection />
      <CESection />
      <FBSection />
      <CESection />
      <FBSection />
      <CESection />
      <FBSection />
      <div className="relative flex">
        <WhiteKey />
      </div>
    </div>
  );
}

function StartSection() {
  return (
    <div className="relative flex">
      {Array.from({ length: 2 }).map((_, i) => (
        <Fragment key={i}>
          <WhiteKey />
          {i !== 1 && <BlackKey multi={i} />}
        </Fragment>
      ))}
    </div>
  );
}

function FBSection() {
  return (
    <div className="relative flex">
      {Array.from({ length: 4 }).map((_, i) => (
        <Fragment key={i}>
          <WhiteKey />
          {i !== 3 && <BlackKey multi={i} />}
        </Fragment>
      ))}
    </div>
  );
}

function CESection() {
  return (
    <div className="relative flex">
      {Array.from({ length: 3 }).map((_, i) => (
        <Fragment key={i}>
          <WhiteKey />
          {i !== 2 && <BlackKey multi={i} />}
        </Fragment>
      ))}
    </div>
  );
}

let increment = window.innerHeight / 700;

function WhiteKey() {
  const [isActive, setIsActive] = useState(false);
  const isActiveRef = useRef(false);
  const keyRef = useRef<HTMLDivElement>(null);

  useEffect(() => {


    function emitBeam() {
      if (!keyRef.current) return;

      const button = keyRef.current;


      let currentHeight = 10;

      const beam = document.createElement("div");
      beam.classList.add("beam");
      document.body.appendChild(beam);

      
      increment = window.innerHeight / 700;
      const buttonRect = button.getBoundingClientRect();
      beam.style.width = `${buttonRect.width}px`;
      beam.style.left = `${buttonRect.left}px`;
      beam.style.bottom = `${buttonRect.height - 10}px`;
      beam.style.translate = "0px 0px";
      beam.style.transition = `translate linear ${2900}ms`;

      requestAnimationFrame(function step() {
        if (isActiveRef.current) {
          beam.style.height = `${currentHeight}px`;
          currentHeight += increment;
          requestAnimationFrame(step);
        } else {

          requestAnimationFrame(() => {
            beam.style.height = `${currentHeight}px`;           
            beam.style.translate = `0px -${window.innerHeight}px`;
          })
    
        setTimeout(() => {
          document.body.removeChild(beam);
          return;
        }, 3000);
        }
      });
    }

    if (isActive) {
      emitBeam();
    }

  }, [isActive]);

  useEffect(() => {
    let keyId = getNextKey();

    document.addEventListener("pianoevent", (ev) => {
      let event = ev as CustomEvent<{
        event_type: string;
        intensity: number;
        key_string: string;
        key_id: number;
      }>;

      if (event.detail.key_id === keyId) {
        if (event.detail.event_type === "KeyPress") {
          setIsActive(true);
          isActiveRef.current = true;
        } else if (event.detail.event_type === "KeyRelease") {
          setIsActive(false);
          isActiveRef.current = false;
        }
      }
    });
  }, []);

  return (
    <div
      className="relative ring-black ring-1 h-full w-[calc(69svw/52)] z-10"
      style={{
        backgroundColor: isActive ? activeColor : "white",
      }}
      ref={keyRef}
    ></div>
  );
}

function BlackKey({ multi }: { multi: number }) {
  const [isActive, setIsActive] = useState(false);
  const prevRef = useRef<HTMLDivElement>(null);
  const nextRef = useRef<HTMLDivElement>(null);
  const isActiveRef = useRef(false);
  const keyRef = useRef<HTMLDivElement>(null);

  useEffect(() => {


    function emitBeam() {
      if (!keyRef.current) return;

      const button = keyRef.current;
      
      let currentHeight = 10;

      const beam = document.createElement("div");
      beam.classList.add("beam");
      document.body.appendChild(beam);

      
      increment = window.innerHeight / 700;
      const buttonRect = button.getBoundingClientRect();
      beam.style.width = `${buttonRect.width}px`;
        beam.style.left = `${buttonRect.left}px`;
      beam.style.bottom = `${buttonRect.height + 25}px`;
      beam.style.translate = "0px 0px";
      beam.style.transition = `translate linear ${2900}ms`;

  
      requestAnimationFrame(function step() {

        if (isActiveRef.current) {
          beam.style.height = `${currentHeight}px`;
          currentHeight += increment;
          requestAnimationFrame(step);
        } else {
         requestAnimationFrame(() => {
            beam.style.height = `${currentHeight}px`;           
            beam.style.translate = `0px -${window.innerHeight}px`;
          })
    
        }
      });
    }

    if (isActive) {
      emitBeam();
    }

  }, [isActive]);

  useEffect(() => {
    let keyId = getNextKey();

    document.addEventListener("pianoevent", (ev) => {
      let event = ev as CustomEvent<{
        event_type: string;
        intensity: number;
        key_string: string;
        key_id: number;
      }>;

      if (event.detail.key_id === keyId) {
        if (event.detail.event_type === "KeyPress") {
          setIsActive(true);
          isActiveRef.current = true;
        } else if (event.detail.event_type === "KeyRelease") {
          setIsActive(false);
          isActiveRef.current = false;
        }
      } else if (event.detail.key_id === keyId - 1) {
        if (event.detail.event_type === "KeyPress") {
          let elem = prevRef.current as HTMLElement;
          elem.style.backgroundColor = activeColor;
        } else if (event.detail.event_type === "KeyRelease") {
          let elem = prevRef.current as HTMLElement;
          elem.style.backgroundColor = "white";
        }
      } else if (event.detail.key_id === keyId + 1) {
        if (event.detail.event_type === "KeyPress") {
          let elem = nextRef.current as HTMLElement;
          elem.style.backgroundColor = activeColor;
        } else if (event.detail.event_type === "KeyRelease") {
          let elem = nextRef.current as HTMLElement;
          elem.style.backgroundColor = "white";
        }
      }
    });
  }, []);

  return (
    <div
      className="h-full w-[calc(69svw/80)]"
      style={{
        left: `calc(69svw/(52*2) + ${multi} * 69svw/52)`,
      }}
    >
      <div className="absolute w-[calc(69svw/80)] h-full z-20">
        <div
          className="w-[calc(69svw/80)] h-3/4 top-0 z-10"
          style={{
            backgroundColor: isActive ? activeColor : "black",
          }}
          ref={keyRef}
        ></div>
        <div className="relative w-[calc(69svw/80)] h-1/4 bottom-0">
          <div
            className="absolute w-[calc(69svw/160+0.5px)] h-full top-0 border-r border-black left-0"
            ref={prevRef}
            style={{
              backgroundColor: "white",
            }}
          ></div>
          <div
            className="absolute w-[calc(69svw/160-0.5px)] h-full top-0 right-0"
            style={{
              backgroundColor: "white",
            }}
            ref={nextRef}
          ></div>
        </div>
      </div>
    </div>
  );
}
