import { debug_print } from "./utils.js";

export function show_error_popup(id, show, message)
{
    const error_message = document.getElementById(id);
    if (show) 
    {
        error_message.style.display = 'block';
        error_message.style.opacity = '1';
        error_message.innerHTML = message;
        
        error_message.classList.add('shake');
        setTimeout(() => 
        {
            error_message.classList.remove('shake');
        }, 500);
        
        setTimeout(() => 
        {
            error_message.style.opacity = '0';
            setTimeout(() => 
            {
                error_message.style.display = 'none';
            }, 500);
        }, 3000);
    } 
    else 
    {
        // Hide the tooltip if the input is valid
        error_message.style.display = 'none';
    }
}