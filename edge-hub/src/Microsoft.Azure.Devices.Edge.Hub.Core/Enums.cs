// Copyright (c) Microsoft. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
namespace Microsoft.Azure.Devices.Edge.Hub.Core
{
    public enum FeedbackStatus
    {
        Complete,
        Abandon,
        Reject
    }

    public enum EndpointType
    {
        Null, // Store
        Cloud,
        Module
    }
}
