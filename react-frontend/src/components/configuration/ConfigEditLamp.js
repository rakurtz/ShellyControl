import './EditShellyLamp.css';

import { Button } from '@mui/material';
import Box from '@mui/material/Box';

import { useEffect, useState } from 'react';


export default function EditLamp(props) {
   // props
    const lamps = props.lamps;
    const setLamps = props.setLamps;
    const handleLampModalClose = props.handleLampModalClose;
    const editLampWithId = props.editLampWithId; // when creating a new one, just give next unused id
    const shellies = props.shellies;

    // determing if new or existing lamp
    const [modifiedLampName, setModifiedLampName] = useState("");
    const [modifiedLampMembers, setModifiedLampMembers] = useState([]);
    
    
    useEffect(() => {
        const existingLamp = lamps.find(lamp => lamp.id === editLampWithId);   
        if (existingLamp) {
            setModifiedLampName(existingLamp.name);
            setModifiedLampMembers(existingLamp.members); // TODO: needs fix to actually show selected options
            setModifiedLampMembers(existingLamp.members);
        }
    }, [editLampWithId]) // (only once)




    // Function to update the shellies with the changed element
    const updateConfigLamps = (modifiedLamp) => {
        // Find the index of the element you want to change
        const index = lamps.findIndex(obj => obj.id === editLampWithId);
        console.log(index)
        var updatedLamps = [];
        if (index !== -1) { // If the element is found
            // Create a new array with the changed element
            updatedLamps = [
                ...lamps.slice(0, index), // Elements before the changed one
                modifiedLamp, // The changed element
                ...lamps.slice(index + 1) // Elements after the changed one
            ];

            // Update the state with the new array
            setLamps(updatedLamps);
        } else {
            // adding new lamp
            updatedLamps = lamps
            updatedLamps.push(modifiedLamp);
            setLamps(updatedLamps);
        }
        console.log(lamps)
    };


    const handleClickOk = () => {
        const modifiedLamp = {
            id: editLampWithId,
            name: modifiedLampName,
            members: modifiedLampMembers
        }
        updateConfigLamps(modifiedLamp)
        setTimeout(() => {          // need short delay here to let EditContainer realize state change of saveCommand
            handleLampModalClose()
        }, 200);
    }
    
    const handleSelect = (e) => {
        const options = e.target.options;
        const modifiedLampOptions = [];
        
        for (let i = 0; i < options.length; i++) {
            if (options[i].selected) {
                const value = JSON.parse(options[i].value);
                modifiedLampOptions.push({device_id: value.device_id, lane: value.lane});
            }
        }
        setModifiedLampMembers(modifiedLampOptions);
    }

    //
    // jsx
    //

    // needed to stringify the object given to "value", cause html needs a string here
    const selectOptions_jsx = shellies.flatMap((shelly) => [
        <option key={`${shelly.id}-0`} value={JSON.stringify({ device_id: shelly.id, lane: 0 })}>{shelly.device_name} - 0</option>,
        <option key={`${shelly.id}-1`} value={JSON.stringify({ device_id: shelly.id, lane: 1 })}>{shelly.device_name} - 1</option>,
        <option key={`${shelly.id}-2`} value={JSON.stringify({ device_id: shelly.id, lane: 2 })}>{shelly.device_name} - 2</option>,
        <option key={`${shelly.id}-3`} value={JSON.stringify({ device_id: shelly.id, lane: 3 })}>{shelly.device_name} - 3</option>
    ]);
    



    // from MUI/Modal (https://mui.com/material-ui/react-modal/)
    const style = {
        position: 'absolute',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        width: '80%',
        maxWidth: '400px',
        bgcolor: 'background.paper',
        border: '2px solid #000',
        boxShadow: 24,
        p: 4,
      };

    return (
        <Box sx={style}>
            <h3>Lamp with ID: {editLampWithId}</h3>
            <table className='edit-table'>
                <tbody>
                <tr>
                    <td>
                        Name:
                    </td>
                    <td>
                        <input className="input-field" name="name" defaultValue={modifiedLampName} onChange={(e) => setModifiedLampName(e.target.value.trim())}/>
                    </td>
                </tr>
                <tr>
                    <td>Members:</td>
                    <td>
                        <select multiple size="8" className="input-field" name="members" onChange={handleSelect} value={modifiedLampMembers.map(member => JSON.stringify(member))}>
                            {selectOptions_jsx}
                        </select>
                       
                    </td>
                </tr>
                </tbody>
            </table>
                        <Button variant="outlined" onClick={handleLampModalClose}>Cancel</Button> &nbsp;
                        <Button variant="contained" onClick={handleClickOk} disabled={!(modifiedLampName.length > 0 && modifiedLampMembers.length > 0)}>Ok</Button>
        </Box>
    )
}