
import './LampContainer.css';
import Lamp from './Lamp';
import { backendUrl } from '../../App';

export default function LampContainer(props) {
    const setLamps = props.setLamps;

    const updateLampsWithLamp = (updatedlamp) => {
        console.log("inside updateLampsWithLamp");
        console.log(updatedlamp);
        setLamps((prevState) => {
            return prevState.map((lamp) => {
                if (lamp.id === updatedlamp.id) {
                    return updatedlamp;
                }
                return lamp;
            });
        });
    }

    const toggleLamp = (id) => {
        console.log(`toggling lamp with name: ${id}`);
        fetch(backendUrl + "/lamp_toggle/" + id)
            .then(res => res.json())
            .then(lamp => updateLampsWithLamp(lamp));       
    }
    
    const switchOnLamp = (id) => {
        console.log(`switching on lamp with name: ${id}`);
        fetch(backendUrl + "/lamp_on/" + id)
            .then(res => res.json())
            .then(lamp => updateLampsWithLamp(lamp));       
    }

    const setBrightnessAndTurnOn = (id, brightness) => {
        console.log(`setting brightness to ${brightness} on lamp with name: ${id}`);
        fetch(backendUrl + "/lamp_set_brightness/" + id + "/" + brightness)
            .then(switchOnLamp(id));
    }

    const commands = {
        toggleLamp: toggleLamp,
        switchOn: switchOnLamp,
        setBrightnessAndTurnOn: setBrightnessAndTurnOn,
        updateLampsWithLamp: updateLampsWithLamp,
    }

    const lampsComponents = props.lamps.map((lamp) => 
                <div key={lamp.id}>
                    <Lamp lamp={lamp} commands={commands}/>
                </div>);
    
    return (
        <div className='flex_container'>
            {lampsComponents}
        </div>
    );  
}