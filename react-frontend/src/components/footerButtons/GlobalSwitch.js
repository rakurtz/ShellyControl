
// Material UI
import { Fab } from '@mui/material';
import FlashlightOffIcon from '@mui/icons-material/FlashlightOffOutlined';

// global import
import { backendUrl } from '../../App';

export default function GlobalSwitch(props) {
    const allLampsAreOff = props.lamps.every(lamp => lamp.is_on === false);

    const switchOffLamps = () => {
        console.log("send api call to switch all off")
        fetch(backendUrl + "/lamps_all_off")
            .then(res => res.json())
            .then(data => props.setLamps(data))
            .catch(error => console.log('Error fetching data:', error));
    }
    
  

    const handleMouseUp = (e) => {
        e.preventDefault();
        if (!allLampsAreOff) {
            switchOffLamps();
        }
    };


    
    // css classes 
       
    return (
        <div>
            <Fab color="secondary" aria-label="allOff" disabled={allLampsAreOff}
                onMouseUp={handleMouseUp} 
                onTouchEnd={handleMouseUp}
                >
                <FlashlightOffIcon fontSize='large'/>
            </Fab>
        </div>
     );
  }