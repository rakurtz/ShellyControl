import './Lamp.css';

import { useState } from 'react';
import Slider from './Slider';

let sliderSensitivity = `${process.env.REACT_APP_SLIDER_SENSITIVITY}`;

if (sliderSensitivity === undefined|| sliderSensitivity < 0 || sliderSensitivity > 10) {
    console.log("Warning: REACT_APP_SLIDER_SENSITIVITY in .env is out of bounds. Setting to 3")
    sliderSensitivity = 3;
}

export default function Lamp(props) {
    const name = props.lamp.name;
    const id = props.lamp.id;
    const commands = props.commands;
    
    const [sliderValue, setSliderValue] = useState(props.lamp.brightness)
    const [isMouseDown, setIsMouseDown] = useState(false);
    const [mouseXMemory, setMouseXMemory] = useState(0);
    const [brightnessChanged, setBrightnessChanged] = useState(false);
    const [showSlider, setShowSlider] = useState(false);
    
    
    const handleMouseDown = (e) => {
        e.preventDefault();
        setIsMouseDown(true);
    };

    const handleMouseUp = (e) => {
        e.preventDefault();
        setIsMouseDown(false);
       
        brightnessChanged
        ? commands.setBrightnessAndTurnOn(id, sliderValue)
        : commands.toggleLamp(id);

        setBrightnessChanged(false);
        setShowSlider(false);
    };

    const handleMouseMove = (e) => {
        e.preventDefault();
        
        // only do somehting when click pressed / touch down
        if (isMouseDown) {
            var clientX = 0;
            // differ between mouse and touch events and extract clientX
            if (typeof e.targetTouches !== 'undefined') {
                clientX = e.targetTouches[0].clientX;
            } else {
                clientX = e.clientX;
            }
                
            // Calculate and set the value based on the mouse movement.
            var newValue = sliderValue;
            if (clientX > mouseXMemory) {
                console.log(sliderSensitivity);
                newValue = sliderValue + 3 // sliderSensitivity;  // 2 or 3 seams to be fine here
                setShowSlider(true)
                setBrightnessChanged(true);
            } else if (clientX < mouseXMemory) {
                newValue = sliderValue - 3 // sliderSensitivity;
                setShowSlider(true)
                setBrightnessChanged(true);
            }

            // limit sliderValue to values between 0 and 100
            if (newValue >= 100) {
                setSliderValue(100);
            } else if (newValue <= 0) {
                setSliderValue(1);
            } else {
                setSliderValue(newValue);
            }

            // keeping track of mouse movement by storing in state
            setMouseXMemory(clientX);
        }
    };
    
    // css classes 
    const pressedClass = isMouseDown ? "Lamps-Single-pressed" : "";
    const isOnClass = props.lamp.is_on ? "" : "Lamps-Single-off";
    const classes = `Lamps-Single ${isOnClass} ${pressedClass}`;

    // conditional rendering of Slider
    const button_label = <div>{name}</div>;
    const slider = showSlider ?
         <Slider value={sliderValue}/> 
         : null;
        

    return (
      <div
        className={classes}
        onMouseDown={handleMouseDown} 
        onMouseUp={handleMouseUp} 
        onMouseMove={handleMouseMove}
        onTouchStart={handleMouseDown}
        onTouchEnd={handleMouseUp}
        onTouchMove={handleMouseMove}
        >
            {slider}
            {button_label}
            <p 
              className={props.lamp.is_on ? "brightness-color-show" : "brightness-color-disabled"}>
                { brightnessChanged? sliderValue : props.lamp.brightness } %
            </p>
      </div>
     );
  }