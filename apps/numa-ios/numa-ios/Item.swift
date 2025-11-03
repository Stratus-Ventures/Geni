//
//  Item.swift
//  numa-ios
//
//  Created by Jason Coawette on 11/2/25.
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
