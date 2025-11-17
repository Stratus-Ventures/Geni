//
//  Item.swift
//  geni-ios
//
//  Created by Jason Coawette on 11/16/25.
//

import Foundation
import SwiftData

@Model
final class Item {
    var timestamp: Date
    
    init(timestamp: Date) {
        self.timestamp = timestamp
    }
}
