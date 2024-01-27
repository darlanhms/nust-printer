/**
 * Enum of the Printer state
 */
#[derive(Debug, Clone)]
pub enum PrinterState {
    /**
     * The printer is able to receive jobs (also idle)
     */
    READY,

    /**
     * The printer is not accepting jobs (also stopped)
     */
    PAUSED,

    /**
     * The printer is now printing an document (also processing)
     */
    PRINTING,

    /**
     * All other status like error, resources, manual intervention, etc...
     */
    UNKNOWN,

}


/**
 * Printer is a struct to representation the system printer
 * They has an ID composed by your system_name and has printing method to print directly
 */
pub struct Printer {
    /**
     * Visual reference of system printer name
     */
    pub name: String,

    /**
     * Name of Printer exactly as on system
     */
    pub system_name: String,

    /**
     * Name of the Printer driver
     */
    pub driver_name: String,


    /**
     * Location definition of printer (default is empty string)
     */
    pub location: String,

    /**
     * Definition if the printer is the default printer
     */
    pub is_default: bool,

    /**
     * Definition if the printer is shared
     */
    pub is_shared: bool,

    /**
     * The state of the printer
     */
    pub state: PrinterState,

}
