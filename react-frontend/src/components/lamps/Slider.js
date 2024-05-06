import './Slider.css';
export default function Slider(props) {
    return (
      <div className="slidecontainer">
        <input type="range" min="1" max="100" value={props.value} readOnly className="slider"/>
    </div>
    );
  }