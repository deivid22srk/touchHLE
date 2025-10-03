/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITableView` and `UITableViewCell`.

use super::ui_scroll_view::UIScrollViewHostObject;
use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_class, msg_super, nil, objc_classes,
    ClassExports, NSZonePtr,
};

// UITableViewStyle
type UITableViewStyle = NSInteger;
#[allow(dead_code)]
const UI_TABLE_VIEW_STYLE_PLAIN: UITableViewStyle = 0;
#[allow(dead_code)]
const UI_TABLE_VIEW_STYLE_GROUPED: UITableViewStyle = 1;

// UITableViewCellStyle
type UITableViewCellStyle = NSInteger;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_STYLE_DEFAULT: UITableViewCellStyle = 0;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_STYLE_VALUE1: UITableViewCellStyle = 1;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_STYLE_VALUE2: UITableViewCellStyle = 2;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_STYLE_SUBTITLE: UITableViewCellStyle = 3;

// UITableViewCellSeparatorStyle
type UITableViewCellSeparatorStyle = NSInteger;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_SEPARATOR_STYLE_NONE: UITableViewCellSeparatorStyle = 0;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_SEPARATOR_STYLE_SINGLE_LINE: UITableViewCellSeparatorStyle = 1;

// UITableViewCellAccessoryType
type UITableViewCellAccessoryType = NSInteger;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_ACCESSORY_NONE: UITableViewCellAccessoryType = 0;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_ACCESSORY_DISCLOSURE_INDICATOR: UITableViewCellAccessoryType = 1;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_ACCESSORY_DETAIL_DISCLOSURE_BUTTON: UITableViewCellAccessoryType = 2;
#[allow(dead_code)]
const UI_TABLE_VIEW_CELL_ACCESSORY_CHECKMARK: UITableViewCellAccessoryType = 3;

pub struct UITableViewHostObject {
    superclass: UIScrollViewHostObject,
    data_source: id,
    delegate: id,
    style: UITableViewStyle,
    separator_style: UITableViewCellSeparatorStyle,
    row_height: f32,
    section_header_height: f32,
    section_footer_height: f32,
    allows_selection: bool,
}
impl_HostObject_with_superclass!(UITableViewHostObject);

impl Default for UITableViewHostObject {
    fn default() -> Self {
        UITableViewHostObject {
            superclass: Default::default(),
            data_source: nil,
            delegate: nil,
            style: UI_TABLE_VIEW_STYLE_PLAIN,
            separator_style: UI_TABLE_VIEW_CELL_SEPARATOR_STYLE_SINGLE_LINE,
            row_height: 44.0,
            section_header_height: 0.0,
            section_footer_height: 0.0,
            allows_selection: true,
        }
    }
}

pub struct UITableViewCellHostObject {
    superclass: super::UIViewHostObject,
    text_label: id,
    detail_text_label: id,
    image_view: id,
    content_view: id,
    accessory_type: UITableViewCellAccessoryType,
    style: UITableViewCellStyle,
    reuse_identifier: id,
    selection_style: NSInteger,
}
impl_HostObject_with_superclass!(UITableViewCellHostObject);

impl Default for UITableViewCellHostObject {
    fn default() -> Self {
        UITableViewCellHostObject {
            superclass: Default::default(),
            text_label: nil,
            detail_text_label: nil,
            image_view: nil,
            content_view: nil,
            accessory_type: UI_TABLE_VIEW_CELL_ACCESSORY_NONE,
            style: UI_TABLE_VIEW_CELL_STYLE_DEFAULT,
            reuse_identifier: nil,
            selection_style: 0, // Blue (default)
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableView: UIScrollView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITableViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame
              style:(UITableViewStyle)style {
    let this: id = msg![env; this initWithFrame:frame];
    if this != nil {
        let host_obj = env.objc.borrow_mut::<UITableViewHostObject>(this);
        host_obj.style = style;
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this != nil {
        log_dbg!("[(UITableView*){:?} initWithCoder:{:?}]", this, coder);
        // TODO: decode properties from coder
    }
    this
}

- (id)dataSource {
    env.objc.borrow::<UITableViewHostObject>(this).data_source
}

- (())setDataSource:(id)data_source {
    use crate::objc::{release, retain};
    
    let old = env.objc.borrow::<UITableViewHostObject>(this).data_source;
    if data_source != nil {
        retain(env, data_source);
    }
    if old != nil {
        release(env, old);
    }
    env.objc.borrow_mut::<UITableViewHostObject>(this).data_source = data_source;
}

- (id)delegate {
    env.objc.borrow::<UITableViewHostObject>(this).delegate
}

- (())setDelegate:(id)delegate {
    use crate::objc::{release, retain};
    
    let old = env.objc.borrow::<UITableViewHostObject>(this).delegate;
    if delegate != nil {
        retain(env, delegate);
    }
    if old != nil {
        release(env, old);
    }
    env.objc.borrow_mut::<UITableViewHostObject>(this).delegate = delegate;
}

- (f32)rowHeight {
    env.objc.borrow::<UITableViewHostObject>(this).row_height
}

- (())setRowHeight:(f32)height {
    env.objc.borrow_mut::<UITableViewHostObject>(this).row_height = height;
}

- (UITableViewCellSeparatorStyle)separatorStyle {
    env.objc.borrow::<UITableViewHostObject>(this).separator_style
}

- (())setSeparatorStyle:(UITableViewCellSeparatorStyle)style {
    env.objc.borrow_mut::<UITableViewHostObject>(this).separator_style = style;
}

- (())reloadData {
    log_dbg!("[(UITableView*){:?} reloadData] (stub)", this);
    // TODO: actually reload data from dataSource
}

- (id)dequeueReusableCellWithIdentifier:(id)identifier { // NSString*
    log_dbg!("[(UITableView*){:?} dequeueReusableCellWithIdentifier:{:?}] (stub - returning nil)", this, identifier);
    // TODO: implement cell reuse pool
    nil
}

- (())dealloc {
    use crate::objc::release;
    
    let (data_source, delegate) = {
        let host_obj = env.objc.borrow_mut::<UITableViewHostObject>(this);
        (host_obj.data_source, host_obj.delegate)
    };
    
    if data_source != nil {
        release(env, data_source);
    }
    if delegate != nil {
        release(env, delegate);
    }
    
    () = msg_super![env; this dealloc]
}

@end

@implementation UITableViewCell: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITableViewCellHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithStyle:(UITableViewCellStyle)style
    reuseIdentifier:(id)reuse_identifier { // NSString*
    use crate::objc::retain;
    
    // TODO: proper frame size
    let frame = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize { width: 320.0, height: 44.0 },
    };
    
    let this: id = msg![env; this initWithFrame:frame];
    if this != nil {
        {
            let host_obj = env.objc.borrow_mut::<UITableViewCellHostObject>(this);
            host_obj.style = style;
            host_obj.reuse_identifier = reuse_identifier;
        }
        
        if reuse_identifier != nil {
            retain(env, reuse_identifier);
        }
        
        // Create content view
        let content_view: id = msg_class![env; UIView alloc];
        let content_view: id = msg![env; content_view initWithFrame:frame];
        
        {
            let host_obj = env.objc.borrow_mut::<UITableViewCellHostObject>(this);
            host_obj.content_view = content_view;
        }
        
        () = msg![env; this addSubview:content_view];
        
        // Create text label
        let text_label: id = msg_class![env; UILabel alloc];
        let text_label: id = msg![env; text_label initWithFrame:frame];
        
        {
            let host_obj = env.objc.borrow_mut::<UITableViewCellHostObject>(this);
            host_obj.text_label = text_label;
        }
        
        () = msg![env; content_view addSubview:text_label];
        
        log_dbg!("[(UITableViewCell*){:?} initWithStyle:{} reuseIdentifier:{:?}]", this, style, reuse_identifier);
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this != nil {
        log_dbg!("[(UITableViewCell*){:?} initWithCoder:{:?}]", this, coder);
        
        // TODO: decode properties from coder
    }
    this
}

- (id)textLabel {
    env.objc.borrow::<UITableViewCellHostObject>(this).text_label
}

- (id)detailTextLabel {
    env.objc.borrow::<UITableViewCellHostObject>(this).detail_text_label
}

- (id)imageView {
    env.objc.borrow::<UITableViewCellHostObject>(this).image_view
}

- (id)contentView {
    env.objc.borrow::<UITableViewCellHostObject>(this).content_view
}

- (UITableViewCellAccessoryType)accessoryType {
    env.objc.borrow::<UITableViewCellHostObject>(this).accessory_type
}

- (())setAccessoryType:(UITableViewCellAccessoryType)accessory_type {
    env.objc.borrow_mut::<UITableViewCellHostObject>(this).accessory_type = accessory_type;
}

- (id)reuseIdentifier {
    env.objc.borrow::<UITableViewCellHostObject>(this).reuse_identifier
}

- (())dealloc {
    use crate::objc::release;
    
    let (text_label, detail_text_label, image_view, content_view, reuse_identifier) = {
        let host_obj = env.objc.borrow_mut::<UITableViewCellHostObject>(this);
        (host_obj.text_label, host_obj.detail_text_label, host_obj.image_view, host_obj.content_view, host_obj.reuse_identifier)
    };
    
    if text_label != nil {
        release(env, text_label);
    }
    if detail_text_label != nil {
        release(env, detail_text_label);
    }
    if image_view != nil {
        release(env, image_view);
    }
    if content_view != nil {
        release(env, content_view);
    }
    if reuse_identifier != nil {
        release(env, reuse_identifier);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
