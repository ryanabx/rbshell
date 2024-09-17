# rbshell - A wayland compositor agnostic desktop shell!

rbshell is an implementation of the desktop metaphor for linux. It currently only contains a panel, but in the future it might contain some other sub projects related to shell components, such as:

- OSDs (Brightness, audio, caps lock, etc.)
- Workspace/virtual desktop overview

For now, we keep the scope limited to a single panel at the bottom of the screen.

> **NOTE:** This project is very early in its development. We are also waiting on a version of iced master that has layer shell support. There is work being done on this at <https://github.com/pop-os/iced> that I have contributed to, basically we need to rebase their changes onto the latest iced master. For now we are using winit until that gets resolved.

## Design Goals

The design goal of this shell is to be as reasonably similar to Windows 11 as possible. The reason for this is to provide a linux shell that feels familiar to users of Windows. The metaphor of the task bar, with a start menu, window list, and system tray are common amongst a lot of desktop shells, but I hope to make something that takes it a step further. Of course, matching Windows 11 one to one would cause users frustration as things won't always function the same, so there's a reasonable amount of difference that we must have.

This project's implementation is meant to be opinionated, and lacks the powerful customization that alternatives might have, but it also serves as a refreshingly simple implementation as a result, eschewing customization in favor of reducing decision fatigue for users who just want to get things done. This comes in contrast to something like [KDE Plasma](https://kde.org/plasma-desktop/)'s panel, or [COSMIC](https://system76.com/cosmic)'s panel, for example, since they both provide a myriad of applets to choose from. This comes with decisions, and speaking personally, I like to keep my variations to a minimum.

A similarly opinionated shell is [GNOME Shell](https://www.gnome.org/); however, the way it implements the desktop metaphor is more "out there", forgoing the traditional Windows-style taskbar entirely. This is exciting for many users, but for others who come from Windows, it's often too much of a transition.

## Support Goals

The goal for this shell is to be compatible with as many compositors as possible, providing the choice for users to choose whatever wayland experience they choose. As part of this goal, we support the following wayland protocols:

- [wlr-layer-shell-unstable-v1](https://wayland.app/protocols/wlr-layer-shell-unstable-v1)
  - This provides the ability for the panel to be anchored at the bottom of the screen, and is a 100% requirement for the shell to work.
  - Supported by Wlroots-based compositors, cosmic-comp, hyprland, and kwin, and many others
  - Not supported by GNOME or Weston
- [wlr-foreign-toplevel-management-unstable-v1](https://wayland.app/protocols/wlr-foreign-toplevel-management-unstable-v1)
  - This provides toplevel list support for wlroots based compositors
  - Supported by wlroots and hyprland, among others
- [cosmic-toplevel-info-unstable-v1](https://wayland.app/protocols/cosmic-toplevel-info-unstable-v1)
  - Provides toplevel list support for cosmic-comp
  - Supported by COSMIC
- [kde-plasma-window-management](https://wayland.app/protocols/kde-plasma-window-management)
  - Provides complete window management (including toplevel list) support for kwin
  - Supported by Kwin
  - > **NOTE:** Only ONE client can bind to this global per session. The KDE desktop typically binds this so you will need to run just Kwin for this global to be bound by rbshell

## Contributing

Contributions are welcome! Please [make an issue](https://github.com/ryanabx/rbshell/issues/) before working on anything, as it's possible we could be duplicating work.

## Reporting bugs

As with above, [making an issue](https://github.com/ryanabx/rbshell/issues/) is highly recommended!