/*---- bEndpointAddress ----*/
pub const USB_EP_EP1_ADDRESS: u8 = USB_EP_OUT_ADDRESS | 0x01;
pub const USB_EP_EP2_ADDRESS: u8 = USB_EP_IN_ADDRESS | 0x01;
pub const USB_EP_EP3_ADDRESS: u8 = USB_EP_OUT_ADDRESS | 0x02;
pub const USB_EP_EP4_ADDRESS: u8 = USB_EP_IN_ADDRESS | 0x02;

/*---- wMaxPacketSize ----*/
pub const USB_EP_MAX_PACKET_SIZE_512: u16 = 0x0200;
pub const USB_EP_MAX_PACKET_SIZE_64: u16 = 0x40;
pub const USB_EP_MAX_PACKET_SIZE_32: u16 = 0x20;
pub const USB_EP_MAX_PACKET_SIZE_16: u16 = 0x10;
pub const USB_EP_MAX_PACKET_SIZE_08: u16 = 0x08;

/*---- bInterval ----*/
pub const USB_EP_CONTROL_INTERVAL: u8 = 0x00;
pub const USB_EP_BULK_INTERVAL: u8 = 0x00;
pub const USB_EP_INTERRUPT_INTERVAL: u8 = 0x10;
pub const USB_EP_ISO_INTERVAL: u8 = 0x00;

pub const USB_EP_EP1_INTERVAL: u8 = USB_EP_BULK_INTERVAL;
pub const USB_EP_EP2_INTERVAL: u8 = USB_EP_BULK_INTERVAL;
pub const USB_EP_EP3_INTERVAL: u8 = USB_EP_INTERRUPT_INTERVAL;
pub const USB_EP_EP4_INTERVAL: u8 = USB_EP_INTERRUPT_INTERVAL;

/*--------------------------------------
    Interface Descriptor Settings
--------------------------------------*/
/*---- bInterfaceNumber ----*/
pub const USB_IF_IF0_NUMBER: u8 = 0;

/*---- bAlternateSettings ----*/
pub const USB_IF_ALT0: u8 = 0;

/*---- bNumEndpoints ----*/
pub const USB_IF_CFG_IF0_NUMBER_EP: u8 = 2;

/*---- bInterfaceClass ----*/
pub const USB_IF0_CLASS_BLD: u8 = 0xff;
pub const USB_IF0_CLASS_MSC: u8 = 0x08;

/*---- bInterfaceSubClass ----*/
pub const USB_IF0_SUBCLASS_BLD: u8 = 0xff;
pub const USB_IF0_SUBCLASS_MSC: u8 = 0x06;

/*---- bInterfaceProtocol ----*/
pub const USB_IF0_PROTOCOL_BLD: u8 = 0;
pub const USB_IF0_PROTOCOL_MSC: u8 = 0x50;

/*---- iInterface ----*/
pub const USB_IF_IDX: u8 = 0;

/*--------------------------------------
    Configuration Descriptor Settings
--------------------------------------*/
pub const USB_CFG_NUMBER_OF_IF : u8 = 1    /* number of interfaces          */;
pub const USB_CFG_VALUE : u8 = 1    /* configuration value           */;
pub const USB_CFG_IDX : u8 = 0    /* configuration string id       */;
pub const USB_CFG_CFG_ATTRIBUES : u8 = 0xc0 /* characteristics               */;
pub const USB_CFG_MAX_POWER : u8 = 100  /* maximum power in 2mA          */;
pub const USB_CFG_TOTAL_LENGTH: u16 = USB_CFG_LENGTH
    + (USB_IF_LENGTH * USB_CFG_NUMBER_OF_IF as u16)
    + (USB_EP_LENGTH * USB_IF_CFG_IF0_NUMBER_EP as u16);

/*--------------------------------------
    device descriptor
--------------------------------------*/
pub const USB_DEV_USB_SPECIFICATION : u16 = 0x0200  /* USB specification      */;
pub const USB_DEV_RELEASE_NUMBER : u16 = 0x0000  /* device release number  */;
pub const USB_DEV_CLASS : u8 = 0x00    /* class code             */;
pub const USB_DEV_SUBCLASS : u8 = 0x00    /* sub-class code         */;
pub const USB_DEV_MSC_CLASS : u8 = 0x00    /* class code             */;
pub const USB_DEV_MSC_SUBCLASS : u8 = 0x00    /* sub-class code         */;
pub const USB_DEV_PROTOCOL : u8 = 0x00    /* protocol code                    */;
/* max packet size for endpoint0*/
pub const USB_DEV_VENDOR_ID : u16 = 0x4255  /* vendor id             */;
pub const USB_DEV_PRODUCT_ID_PUD_BLD : u16 = 0x0001  /* product id            */;
pub const USB_DEV_PRODUCT_ID_PUD_MSC : u16 = 0x1000  /* product id            */;
pub const USB_DEV_RELASE_NUMBER : u16 = 0x00    /* device release number */;
pub const USB_DEV_MANUFACTURER_IDX : u8 = 0x01    /* manifacturer string id */;
pub const USB_DEV_PRODUCT_IDX : u8 = 0x02    /* product string id      */;
pub const USB_DEV_SERIAL_NUMBER_IDX : u8 = 0x03    /* serial number string id */;
pub const USB_DEV_NUM_CONFIG : u8 = 1       /* number of possible configure */;

pub const fn lb16(val: u16) -> u8 {
    (val & 0xFF) as u8
}
pub const fn hb16(val: u16) -> u8 {
    ((val >> 8) & 0xFF) as u8
}

/*====================================================
    desciptor relation definition
====================================================*/
/*---- number of configration ----*/
pub const USB_NUM_CONFIG: u8 = USB_DEV_NUM_CONFIG;

/*---- number of interface ----*/
pub const USB_NUM_INTERFACE: u8 = USB_CFG_NUMBER_OF_IF;

/*---- configuration descriptor total size ----*/
pub const USB_CONFIG_DESC_TOTAL_SIZE: u16 = USB_CFG_TOTAL_LENGTH;

/*---- default configuration number ----*/
pub const USB_DEFAULT_CONFIG: u8 = 1;

//----		 end		--------------------

/*--------------------------------------
    USB Request Type
--------------------------------------*/
pub const USB_DEV_REQ_TYPE_STANDARD : u8 = 0x00    /* standard request         */;
pub const USB_DEV_REQ_TYPE_CLASS : u8 = 0x01    /* class specific request   */;
pub const USB_DEV_REQ_TYPE_VENDER : u8 = 0x02    /* vendor specific request  */;
pub const USB_DEV_REQ_TYPE_RESERVE : u8 = 0x03    /* reserved                 */;
pub const USB_DEV_REQ_TYPE_TYPE: u8 = 0x60;
pub const USB_DEV_REQ_DIRECTION: u8 = 0x80;
pub const USB_DEV_REQ_TYPE_UNSUPPORTED : u8 = 0xff    /* unsupported              */;

/*--------------------------------------
    USB Standard Device Request
--------------------------------------*/
pub const USB_GET_STATUS : u8 = 0   /* GetStatus request            */;
pub const USB_CLEAR_FEATURE : u8 = 1   /* ClearFeature request         */;
pub const USB_SET_FEATURE : u8 = 3   /* SetFeature request           */;
pub const USB_SET_ADDRESS : u8 = 5   /* SetAddress request           */;
pub const USB_GET_DESCRIPTOR : u8 = 6   /* GetDescriptor request        */;
pub const USB_SET_DESCRIPTOR : u8 = 7   /* SetDescriptor request        */;
pub const USB_GET_CONFIGURATION : u8 = 8   /* GetConfiguratoin request     */;
pub const USB_SET_CONFIGURATION : u8 = 9   /* SetConfiguratoin request     */;
pub const USB_GET_INTERFACE : u8 = 10   /* GetInterface request         */;
pub const USB_SET_INTERFACE : u8 = 11   /* SetInterface request         */;
pub const USB_SYNCH_FRAME : u8 = 12   /* SynchFrame request           */;

/*--------------------------------------
    device request
--------------------------------------*/
pub const USB_DEVICE_REQUEST_SIZE : u8 = 8       /* device request size      */;
pub const USB_DEVICE_TO_HOST : u8 = 0x80    /* device to host transfer  */;
pub const USB_HOST_TO_DEVICE : u8 = 0x00    /* host to device transfer  */;
pub const USB_DEVICE : u8 = 0x00    /* request to device        */;
pub const USB_INTERFACE : u8 = 0x01    /* request to interface     */;
pub const USB_ENDPOINT : u8 = 0x02    /* request to endpoint      */;
pub const USB_CLASS : u8 = 0x20    /* class request            */;
pub const USB_VENDOR : u8 = 0x40    /* vendor request           */;

/*--------------------------------------
    descriptor size
--------------------------------------*/
pub const USB_DEV_LENGTH : u16 = 0x12    /* device descriptor size       */;
pub const USB_CFG_LENGTH : u16 = 0x09    /* config descriptor size       */;
pub const USB_IF_LENGTH : u16 = 0x09    /* interface descriptor size    */;
pub const USB_EP_LENGTH : u16 = 0x07    /* endpoint descriptor size     */;
pub const USB_DEV_QUALIFIER_LENGTH: u16 = 0x0a;
/*--------------------------------------
    endpoint address
--------------------------------------*/
pub const USB_EP_IN_ADDRESS : u8 = 0x80    /* IN endpoint address          */;
pub const USB_EP_OUT_ADDRESS : u8 = 0x00    /* OUT endpoint address         */;

/*--------------------------------------
    endpoint attribute
--------------------------------------*/
pub const USB_EP_ATR_CONTROL : u8 = 0x00    /* transfer mode : control      */;
pub const USB_EP_ATR_ISO : u8 = 0x01    /* transfer mode : isochronous  */;
pub const USB_EP_ATR_BULK : u8 = 0x02    /* transfer mode : bulk         */;
pub const USB_EP_ATR_INTERRUPT : u8 = 0x03    /* transfer mode : interrupt    */;

/*--------------------------------------
    USB Feature Selector
--------------------------------------*/
pub const USB_DEVICE_REMOTE_WAKEUP : u8 = 1   /* remote wake up               */;
pub const USB_ENDPOINT_STALL : u8 = 0   /* endpoint stall               */;
pub const USB_ENDPOINT_HALT: u8 = USB_ENDPOINT_STALL;
/*--------------------------------------
    USB descriptor type
--------------------------------------*/
pub const USB_DEVICE_DESCRIPTOR : u8 = 1   /* device descriptor            */;
pub const USB_CONFIGURATION_DESCRIPTOR : u8 = 2   /* configuraton descriptor      */;
pub const USB_STRING_DESCRIPTOR : u8 = 3   /* string descriptor            */;
pub const USB_INTERFACE_DESCRIPTOR : u8 = 4   /* interface descriptor         */;
pub const USB_ENDPOINT_DESCRIPTOR : u8 = 5   /* endpoint descriptor          */;
pub const USB_DEVICE_QUALIFIER: u8 = 6;
pub const USB_OTHER_SPEED_CONFIGURATION: u8 = 7;

/*--------------------------------------
    config descriptor definitions
--------------------------------------*/
pub const USB_DEVDESC_ATB_BUS_POWER : u8 = 0x80    /* bus power                */;
pub const USB_DEVDESC_ATB_SELF_POWER : u8 = 0x40    /* self power               */;
pub const USB_DEVDESC_ATB_RMT_WAKEUP : u8 = 0x20    /* remote wake up           */;

pub const DATA_USB_DEVICE_DESCRIPTOR: [u8; 18] = [
    USB_DEV_LENGTH as u8,             /* descriptor size             */
    USB_DEVICE_DESCRIPTOR,            /* descriptor type             */
    lb16(USB_DEV_USB_SPECIFICATION),  /* USB specification           */
    hb16(USB_DEV_USB_SPECIFICATION),  /* USB specification           */
    USB_DEV_CLASS,                    /* class code                  */
    USB_DEV_SUBCLASS,                 /* sub class code              */
    USB_DEV_PROTOCOL,                 /* protocol code               */
    USB_EP_MAX_PACKET_SIZE_64 as u8,  /* max packet size for endpoint 0       */
    lb16(USB_DEV_VENDOR_ID),          /* vendor id                            */
    hb16(USB_DEV_VENDOR_ID),          /* vendor id                            */
    lb16(USB_DEV_PRODUCT_ID_PUD_BLD), /* product id                           */
    hb16(USB_DEV_PRODUCT_ID_PUD_BLD), /* product id                           */
    lb16(USB_DEV_RELASE_NUMBER),      /* device release number                */
    hb16(USB_DEV_RELASE_NUMBER),      /* device release number                */
    1,                                /* manifacturer string id               */
    2,                                /* product string id                    */
    3,                                /* serial number string id              */
    USB_DEV_NUM_CONFIG,               /* number of possible configuration     */
];

pub const DATA_USB_DEVICE_QUALIFIER: [u8; 10] = [
    USB_DEV_QUALIFIER_LENGTH as u8,  /* descriptor size            */
    USB_DEVICE_QUALIFIER,            /* descriptor type            */
    lb16(USB_DEV_USB_SPECIFICATION), /* USB specification          */
    hb16(USB_DEV_USB_SPECIFICATION), /* USB specification          */
    USB_DEV_CLASS,                   /* class code                 */
    USB_DEV_SUBCLASS,                /* sub class code             */
    USB_DEV_PROTOCOL,                /* protocol code                        */
    USB_EP_MAX_PACKET_SIZE_64 as u8, /* max packet size for endpoint 0       */
    USB_DEV_NUM_CONFIG,              /* number of possible configuration     */
    0,
];

pub const DATA_USB_CONFIGURATION_DESCRIPTOR: [u8; 46] = [
    /* configuration1 */
    USB_CFG_LENGTH as u8,         /* descriptor size           */
    USB_CONFIGURATION_DESCRIPTOR, /* descriptor type           */
    lb16(USB_CFG_TOTAL_LENGTH),   /* total length              */
    hb16(USB_CFG_TOTAL_LENGTH),
    USB_CFG_NUMBER_OF_IF,  /* number of interface       */
    USB_CFG_VALUE,         /* configuration value       */
    USB_CFG_IDX,           /* configuration string id   */
    USB_CFG_CFG_ATTRIBUES, /* characteristics           */
    USB_CFG_MAX_POWER,     /* maximum power in 2mA      */
    /* interface0 alt0 */
    USB_IF_LENGTH as u8,      /* descriptor size          */
    USB_INTERFACE_DESCRIPTOR, /* descriptor type          */
    USB_IF_IF0_NUMBER,        /* interface number         */
    USB_IF_ALT0,              /* alternate setting        */
    USB_IF_CFG_IF0_NUMBER_EP, /* number of endpoint       */
    USB_IF0_CLASS_BLD,        /* interface class          */
    USB_IF0_SUBCLASS_BLD,     /* interface sub-class      */
    USB_IF0_PROTOCOL_BLD,     /* interface protocol       */
    USB_IF_IDX,               /* interface string id      */
    /* endpoint descriptors */
    /* 1 */
    USB_EP_LENGTH as u8,              /* descriptor size      */
    USB_ENDPOINT_DESCRIPTOR,          /* descriptor type      */
    USB_EP_EP1_ADDRESS,               /* endpoint address     */
    USB_EP_ATR_BULK,                  /* character address    */
    lb16(USB_EP_MAX_PACKET_SIZE_512), /* max packet size      */
    hb16(USB_EP_MAX_PACKET_SIZE_512),
    USB_EP_EP1_INTERVAL, /* polling interval     */
    /* 2 */
    USB_EP_LENGTH as u8,              /* descriptor size      */
    USB_ENDPOINT_DESCRIPTOR,          /* descriptor type      */
    USB_EP_EP2_ADDRESS,               /* endpoint address     */
    USB_EP_ATR_BULK,                  /* character address    */
    lb16(USB_EP_MAX_PACKET_SIZE_512), /* max packet size      */
    hb16(USB_EP_MAX_PACKET_SIZE_512),
    USB_EP_EP2_INTERVAL, /* polling interval     */
    /* 3 */
    USB_EP_LENGTH as u8,             /* descriptor size      */
    USB_ENDPOINT_DESCRIPTOR,         /* descriptor type      */
    USB_EP_EP3_ADDRESS,              /* endpoint address     */
    USB_EP_ATR_INTERRUPT,            /* character address    */
    lb16(USB_EP_MAX_PACKET_SIZE_64), /* max packet size      */
    hb16(USB_EP_MAX_PACKET_SIZE_64),
    USB_EP_EP3_INTERVAL, /* polling interval     */
    /* 4 */
    USB_EP_LENGTH as u8,             /* descriptor size      */
    USB_ENDPOINT_DESCRIPTOR,         /* descriptor type      */
    USB_EP_EP4_ADDRESS,              /* endpoint address     */
    USB_EP_ATR_INTERRUPT,            /* character address    */
    lb16(USB_EP_MAX_PACKET_SIZE_64), /* max packet size      */
    hb16(USB_EP_MAX_PACKET_SIZE_64),
    USB_EP_EP4_INTERVAL, /* polling interval     */
];

pub const DATA_USB_STRING_DESCRIPTOR_0: [u8; 4] = [
    4,                     /* size of String Descriptor        */
    USB_STRING_DESCRIPTOR, /* String Descriptor type           */
    0x09,
    0x04, /*  Primary/Sub LANGID              */
];

pub const DATA_USB_STRING_DESCRIPTOR_1: [u8; 10] = [
    10,                    /* size of String Descriptor        */
    USB_STRING_DESCRIPTOR, /* String Descriptor type           */
    b't',
    0x00,
    b'm',
    0x00,
    b'b',
    0x00, /* "AMBA"                           */
    b'!',
    0x00,
];

/* for bootloader Class */
pub const DATA_USB_STRING_DESCRIPTOR_2: [u8; 56] = [
    56,                    /* size of String Descriptor        */
    USB_STRING_DESCRIPTOR, /* String Descriptor type           */
    b'A',
    0x00,
    b'm',
    0x00,
    b'b',
    0x00, /* "Ambarella USB generic class"    */
    b'a',
    0x00,
    b'r',
    0x00,
    b'e',
    0x00,
    b'l',
    0x00,
    b'l',
    0x00,
    b'a',
    0x00,
    b' ',
    0x00,
    b'U',
    0x00,
    b'S',
    0x00,
    b'B',
    0x00,
    b' ',
    0x00,
    b'g',
    0x00,
    b'e',
    0x00,
    b'n',
    0x00,
    b'e',
    0x00,
    b'r',
    0x00,
    b'i',
    0x00,
    b'c',
    0x00,
    b' ',
    0x00,
    b'c',
    0x00,
    b'l',
    0x00,
    b'a',
    0x00,
    b's',
    0x00,
    b's',
    0x00,
];

pub const DATA_USB_STRING_DESCRIPTOR_3: [u8; 26] = [
    26,                    /* size of String Descriptor        */
    USB_STRING_DESCRIPTOR, /* String Descriptor type           */
    b'1',
    0x00,
    b'2',
    0x00,
    b'3',
    0x00, /*  "123456789ABC"                  */
    b'4',
    0x00,
    b'5',
    0x00,
    b'6',
    0x00,
    b'7',
    0x00,
    b'8',
    0x00,
    b'9',
    0x00,
    b'A',
    0x00,
    b'B',
    0x00,
    b'C',
    0x00,
];
